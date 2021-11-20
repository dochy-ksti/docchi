use crate::kval_enum::KVal;
use anyhow::Result;
use crate::enc_dec::encode::encode;

/// More user-friendly version of encode
pub fn encode_to_vec(input : &[KVal]) -> Result<Vec<u8>>{
    let mut vec : Vec<u8> = Vec::new();
    let _ = encode(input, &mut vec)?;
    Ok(vec)
}