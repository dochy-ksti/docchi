use crate::kval_enum::KVal;
use crate::enc_dec::decode::decode;
use crate::error::ComResult;

/// &mut &vec may not be user-friendly.
pub fn decode_from_slice(slice : &[u8]) -> ComResult<(Vec<KVal>, usize)>{
    let mut slice = slice;
    decode(&mut slice)
}