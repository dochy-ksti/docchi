use crate::kval_enum::KVal;
use anyhow::Result;
use crate::enc_dec::decode::decode;

/// &mut &vec may not be user-friendly.
pub fn decode_from_slice(slice : &[u8]) -> Result<(Vec<KVal>, usize)>{
    let mut slice = slice;
    decode(&mut slice)
}