use crate::kval_enum::KVal;
use crate::enc_dec::encode::encode;
use crate::error::ComResult;

/// More user-friendly version of encode
pub fn encode_to_vec(input : &[KVal]) -> ComResult<Vec<u8>>{
    let mut vec : Vec<u8> = Vec::new();
    let _ = encode(input, &mut vec)?;
    Ok(vec)
}