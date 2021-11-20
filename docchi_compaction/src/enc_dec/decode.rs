use crate::kval_enum::{KVal};
use super::tag_reader::TagReader;
use crate::enc_dec::kihon_from_tag::KihonFromTag;
use anyhow::Result;
use std::io::Read;
use crate::enc_dec::reader::Reader;
use with_capacity_safe::vec_with_capacity_safe;

/// Decode the bytes to KVals and return them with bytes_read.
pub fn decode<R : Read>(read : &mut R) -> Result<(Vec<KVal>, usize)>{
    let mut reader = Reader::new(read);
    let (count, _) = super::var_int::read(&mut reader)?;
    let count = count as usize;
    let (tag_len, _) = super::var_int::read(&mut reader)?;
    let tag_len = tag_len as usize;

    let mut tr = TagReader::new(reader.read(tag_len)?);
    let mut tags : Vec<KihonFromTag> = vec_with_capacity_safe(count)?;
    for _ in 0..count{
        tags.push(tr.read_tag()?);
    }

    let mut result : Vec<KVal> = vec_with_capacity_safe(count)?;
    for tag in tags{
        let kihon = match tag{
            KihonFromTag::Null => KVal::Null,
            KihonFromTag::Bit(b) => KVal::Bit(b),
            KihonFromTag::Bool(b) => KVal::Bool(b),
            KihonFromTag::Byte => KVal::Byte(reader.read_byte()? as i8),
            KihonFromTag::Str16(l) =>{
                KVal::Str16(reader.read_string(l as usize )?)
            },
            KihonFromTag::Int(size) => {
                let size = size as usize;
                let bytes = reader.read(size)?;
                KVal::Int(super::var_int::decode(bytes))
            },
            KihonFromTag::Float =>{
                let v = reader.read(4)?;
                let f : f32 = f32::from_be_bytes([v[0],v[1],v[2],v[3]]);
                KVal::Float(f)
            },
            KihonFromTag::Str256 =>{
                let size = reader.read_byte()? as usize;
                KVal::Str256(reader.read_string(size)?)
            },
            KihonFromTag::Double =>{
                let v = reader.read(8)?;
                let d : f64 = f64::from_be_bytes([v[0],v[1],v[2],v[3],v[4],v[5],v[6],v[7]]);
                KVal::Double(d)
            },
            KihonFromTag::Decimal(size) =>{
                let size = size as usize;
                let vec = reader.read(size)?;
                let v = super::var_int::decode128(vec);
                let dot = reader.read_byte()?;
                KVal::Decimal(crate::kval_enum::Decimal::new(v, dot))
            },
            KihonFromTag::BigStr(size) =>{
                let v = reader.read(size as usize)?;
                let len = super::var_int::decode(v);
                let s = reader.read_string(len as usize)?;
                KVal::BigStr(s)
            },
            KihonFromTag::Binary(size) =>{
                let v = reader.read(size as usize)?;
                let len = super::var_int::decode(v) as usize;
                let v = reader.read_binary(len)?;
                KVal::Binary(v)
            },
            KihonFromTag::Binary8(size) =>{
                let v = reader.read(size as usize)?;
                let len = super::var_int::decode(v) as usize;
                let v = reader.read_binary8(len)?;
                KVal::Binary8(v)
            },
            KihonFromTag::Binary4(size) =>{
                let v = reader.read(size as usize)?;
                let len = super::var_int::decode(v) as usize;
                let v = reader.read_binary4(len)?;
                KVal::Binary4(v)
            },
            KihonFromTag::Binary2(size) =>{
                let v = reader.read(size as usize)?;
                let len = super::var_int::decode(v) as usize;
                let v = reader.read_binary2(len)?;
                KVal::Binary2(v)
            },
            KihonFromTag::Undefined(i) => KVal::Undefined(i),
        };
        result.push(kihon);
    }

    return Ok((result, reader.bytes_read()));
}
