use crate::kval_enum::KVal;
use crate::enc_dec::tag_storage::TagStorage;
use std::io::Write;
use anyhow::Result;
use crate::enc_dec::writer::Writer;

/// Serialize KVals and return bytes_written.
pub fn encode<W : Write>(vec : &[KVal], write : &mut W) -> Result<usize>{
    let mut writer = Writer::new(write);

    //どんぶり勘定
    let mut data : Vec<u8> = Vec::with_capacity(vec.len() * 2);
    let mut tag = TagStorage::new();
    for item in vec{
        match item{
            KVal::Null =>{ tag.append(0b0100_1, 5) }
            KVal::Bit(b) =>{ tag.append(0b10 + *b as u64, 2) }
            KVal::Bool(b) =>{ tag.append(0b0100_010 + *b as u64, 7) }
            KVal::Byte(i) =>{
                tag.append(0b011, 3);
                data.push(*i as u8);
            },
            KVal::Str16(s) =>{
                tag.append(0b0101, 4);
                if s.len() < 16 {
                    tag.append(s.len() as u64, 4);
                } else{
                    panic!("Small str's len must be within 15");
                }
                data.extend_from_slice(s.as_bytes());
            },
            KVal::Int(i) => {
                let ans = super::var_int::encode(*i);
                let size = ans.len();
                if size == 0 || 9 <= size { unreachable!("Int's size must be 1..=8"); }
                tag.append(0b0011, 4);
                tag.append((size - 1) as u64, 3);
                data.extend_from_slice(&ans);
            },
            KVal::Float(f) =>{
                tag.append(0b0010_0001, 8);
                data.extend_from_slice(&f.to_be_bytes());
            },
            KVal::Str256(s) =>{
                tag.append(0b0010_1, 5);
                if 256 <= s.len(){
                    panic!("Str256's len must be 0..=255")
                }
                data.push(s.len() as u8);
                data.extend_from_slice(s.as_bytes());
            },
            KVal::Double(d) =>{
                tag.append(0b0010_01, 6);
                data.extend_from_slice(&d.to_be_bytes());
            },
            KVal::Decimal(d) => {
                let i = d.int;
                let dot = d.dot;
                let ans = super::var_int::encode128(i);
                let size = ans.len();
                if size == 0 || 17 <= size { unreachable!("decimal's size must be 1..=16"); }

                tag.append(0b0100_001, 7);
                tag.append((size - 1) as u64, 4);
                data.extend_from_slice(&ans);
                data.push(dot);
            }
            KVal::BigStr(s)=>{
                tag.append(0b0010_001, 7);
                let vec = super::var_int::encode(s.len() as i64);
                //if 9 <= vec.len(){ panic!("BigStr is too large"); }
                tag.append((vec.len() - 1) as u64, 3);
                data.extend_from_slice(&vec);
                data.extend_from_slice(s.as_bytes());
            },
            KVal::Binary(v) =>{
                tag.append(0b0001_1, 5);
                //ここは本当はu64でencodeすべき。気が向いたら直したい
                let vec = super::var_int::encode(v.len() as i64);
                tag.append((vec.len() - 1) as u64, 3);
                data.extend_from_slice(&vec);
                data.extend_from_slice(v);
            },
            KVal::BinaryArc(v) =>{
                //let v : &Vec<u8> = v;
                tag.append(0b0001_1, 5);
                //ここは本当はu64でencodeすべき。気が向いたら直したい
                let vec = super::var_int::encode(v.len() as i64);
                tag.append((vec.len() - 1) as u64, 3);
                data.extend_from_slice(&vec);
                data.extend_from_slice(v);
            },
            KVal::Binary8(v) =>{
                tag.append(0b0001_01, 6);
                let vec = super::var_int::encode(v.len() as i64);
                tag.append((vec.len() - 1) as u64, 3);
                data.extend_from_slice(&vec);
                for i in v{
                    data.write_all(&i.to_le_bytes())?;
                }
            },
            KVal::Binary4(v) =>{
                tag.append(0b0001_001, 7);
                let vec = super::var_int::encode(v.len() as i64);
                tag.append((vec.len() - 1) as u64, 3);
                data.extend_from_slice(&vec);
                for i in v{
                    data.write_all(&i.to_le_bytes())?;
                }
            },
            KVal::Binary2(v) =>{
                tag.append(0b0001_0001, 8);
                let vec = super::var_int::encode(v.len() as i64);
                tag.append((vec.len() - 1) as u64, 3);
                data.extend_from_slice(&vec);
                for i in v{
                    data.write_all(&i.to_le_bytes())?;
                }
            },
            KVal::Undefined(i)=>{
                if 8 <= *i{ panic!("Undefined must be 0..=7") }
                tag.append(0b0000_1, 5);
                tag.append(0, *i as usize);
                tag.append(1, 1);
            },

        }
    }
    let tag_vec = tag.to_vec();

    super::var_int::write(vec.len() as i64, &mut writer)?;
    super::var_int::write(tag_vec.len() as i64, &mut writer)?;
    writer.write(&tag_vec)?;
    writer.write(&data)?;
    //writer.flush()?;
    return Ok(writer.bytes_written())
}
