use bit_vec::BitVec;
use crate::enc_dec::kihon_from_tag::KihonFromTag;
use crate::error::ComResult;

pub(crate) struct TagReader{
    pub(crate) vec : BitVec,
    pub(crate) index : usize,
}

impl TagReader{
    pub(crate) fn new(bytes : &[u8]) -> TagReader{
        TagReader{ vec : BitVec::from_bytes(bytes), index : 0 }
    }

    pub(crate) fn read_bit(&mut self) -> ComResult<bool>{
        if self.index < self.vec.len() {
            let r = self.vec[self.index];
            self.index += 1;
            return Ok(r);
        } else{
            Err("read_bit failed")?
        }
    }

    pub(crate) fn read_bits(&mut self, size : usize) -> ComResult<u64>{
        let mut r : u64 = 0;
        for _ in 0..size{
            r *= 2;
            r += self.read_bit()? as u64;
        }
        return Ok(r);
    }

    pub(crate) fn read_next_1(&mut self) -> ComResult<usize>{
        let mut count = 0;
        loop{
            let b = self.read_bit()?;
            if b{ return Ok(count); }
            count += 1;
        }
    }

    pub(crate) fn read_tag(&mut self) -> ComResult<KihonFromTag>{
        let count = self.read_next_1()?;
        match count{
            0 => return Ok(KihonFromTag::Bit(self.read_bit()?)),
            1 =>{
                let count = self.read_next_1()?;
                match count{
                    0 => return Ok(KihonFromTag::Byte),
                    1 => return Ok(KihonFromTag::Str16(self.read_bits(4)? as u8)),
                    2 => return Ok(KihonFromTag::Null),
                    3 => return Ok(KihonFromTag::Bool(self.read_bit()?)),
                    4 => {
                        let bytes = self.read_bits(4)?;
                        return Ok(KihonFromTag::Decimal((bytes + 1) as u8));
                    },
                    _ =>{ Err(format!("undefined tag 01 count {}", count))? }
                }
            }
            2 => {
                let count = self.read_next_1()?;
                match count{
                    0 => {
                        let bytes = self.read_bits(3)?;
                        return Ok(KihonFromTag::Int((bytes + 1) as u8));
                    },
                    1 => return Ok(KihonFromTag::Str256),
                    2  => return Ok(KihonFromTag::Double),
                    3 => {
                        let bytes = self.read_bits(3)?;
                        return Ok(KihonFromTag::BigStr((bytes + 1) as u8));
                    },
                    4 => return Ok(KihonFromTag::Float),
                    _ => Err(format!("undefined tag 001 count {}", count))?,
                }
            },
            3 =>{
                let count = self.read_next_1()?;
                match count{
                    0 => {
                        let bytes = self.read_bits(3)?;
                        return Ok(KihonFromTag::Binary((bytes + 1) as u8));
                    },
                    1 => {
                        let bytes = self.read_bits(3)?;
                        return Ok(KihonFromTag::Binary8((bytes + 1) as u8));
                    },
                    2 => {
                        let bytes = self.read_bits(3)?;
                        return Ok(KihonFromTag::Binary4((bytes + 1) as u8));
                    },
                    3 => {
                        let bytes = self.read_bits(3)?;
                        return Ok(KihonFromTag::Binary2((bytes + 1) as u8));
                    },
                    _ => Err(format!("undefined tag 0001 count {}", count))?,
                }
            }
            4 =>{
                let count = self.read_next_1()?;
                return Ok(KihonFromTag::Undefined(count as u8));
            },
            _ =>{ Err("undefined_tag 00000")? } //panic!("Tag's zeroes must be within 3") }
        }
    }

    // pub fn rest_of_the_vec(self) -> Vec<u8>{
    //     let mut index = self.index / 8;
    //     if self.index % 8 != 0{ index += 1; }
    //     Vec::from(&self.vec.to_bytes()[index..])
    // }
}