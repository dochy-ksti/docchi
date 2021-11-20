use super::signed_bytes::{signed_bytes, signed_bytes128};
use arrayvec::ArrayVec;
use anyhow::{Result};
use std::io::{Write, Read};
use crate::enc_dec::writer::Writer;
use crate::enc_dec::reader::Reader;

///compact i64 as small as possible。(0 needs 1 byte.)
pub fn encode(val : i64) -> ArrayVec<[u8; 8]>{
    let sign;
    let value;
    if val < 0{
        sign = false;
        //127..-128 のような感じなので、マイナスの方が一つ多い。そこを補正しつつ-1もかけることで、0-127に収める。
        value = ((val + 1) * -1) as u64;
    }  else{
        sign = true;
        value = val as u64;
    }
    let signed_bytes = signed_bytes(val);
    let mut bytes = to_bytes(value);

    if bytes.len() < signed_bytes{ bytes.push(0); }

    if sign == false{
        (*bytes.last_mut().unwrap()) |= 0b1000_0000; //負の場合左端を1に
    }
    return bytes;
}

/// decode compacted bytes to i64
pub fn decode(vec : &[u8]) -> i64{
    if vec.len() == 0{ return 0; }
    let mut ans = from_bytes(&vec) as i64;
    let sign_bit : i64 = 0b1000_0000 << (8 * (vec.len() - 1));
    if sign_bit & ans != 0{
        ans ^= sign_bit;
        ans = (ans * -1) - 1;
    }
    return ans;
}

/// compact u64 to bytes.
pub fn to_bytes(val : u64) -> ArrayVec<[u8; 8]>{
    let mut r = ArrayVec::new();
    if val == 0 {
        r.push(0);
        return r;
    }
    let mut val = val;

    loop{
        if val == 0{ break; }
        r.push((val % 256) as u8);
        val = val / 256;
    }
    return r;
}

/// compact u128 to bytes.
pub fn to_bytes128(val : u128) -> ArrayVec<[u8; 16]>{
    let mut r = ArrayVec::new();
    if val == 0{ r.push(0); return r; }
    let mut val = val;

    loop{
        if val == 0{ break; }
        r.push((val % 256) as u8);
        val = val / 256;
    }
    return r;
}

/// decode compacted bytes to u64
pub fn from_bytes(vec : &[u8]) -> u64{
    let mut result : u64 = 0;
    for i in vec.iter().rev(){
        result *= 256;
        result += (*i) as u64;
    }
    return result;
}

/// decode compacted bytes to u128
pub fn from_bytes128(vec : &[u8]) -> u128{
    let mut result : u128 = 0;
    for i in vec.iter().rev(){
        result *= 256;
        result += (*i) as u128;
    }
    return result;
}

/// compact i128 to bytes.
pub fn encode128(val : i128) -> ArrayVec<[u8; 16]>{
    let sign;
    let uval;
    if val < 0{
        sign = false;
        uval = ((val +1) * -1) as u128;
    } else{
        sign = true;
        uval = val as u128;
    }
    let signed_bytes = signed_bytes128(val);
    let mut bytes = to_bytes128(uval);

    if bytes.len() < signed_bytes{ bytes.push(0); }

    if sign == false{
        (*bytes.last_mut().unwrap())  |= 0b1000_0000;
    }
    return bytes;
}

/// decode compacted bytes to i128.
pub fn decode128(vec : &[u8]) -> i128{
    if vec.len() == 0{ return 0; }
    let mut ans = from_bytes128(&vec) as i128;
    let sign_bit : i128 =  0b1000_0000 << (8 * (vec.len() - 1));
    if sign_bit & ans != 0{
        ans ^= sign_bit;
        ans = (ans * -1) - 1;
    }
    return ans;
}


pub(crate) fn write<T : Write>(int : i64, writer : &mut Writer<T>) -> Result<usize>{
    let bytes = super::var_int::encode(int);
    let bytes_len = bytes.len();
    writer.write_byte(bytes_len as u8)?;
    writer.write(bytes.as_slice())?;
    return Ok(bytes_len + 1);
}

pub(crate) fn read<T : Read>(reader : &mut Reader<T>) -> Result<(i64, usize)> {
    let bytes_len = reader.read_byte()? as usize;
    let bytes = reader.read(bytes_len)?;
    Ok((super::var_int::decode(bytes), bytes_len + 1))
}