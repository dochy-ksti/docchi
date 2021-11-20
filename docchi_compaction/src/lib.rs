
/// Define the value type to serialize
pub mod kval_enum;

/// Details about the encoding and the decoding
pub mod enc_dec;

/// Convert bytes to the string which can be used as a part of URL and vice versa
pub mod url_string;

/// Compact integers and booleans
pub mod basic_compaction;

/// Compact strings if the string represents a number
pub mod string_compaction;

#[allow(unused_imports, dead_code)]
#[cfg(test)]
mod testing;



pub use crate::enc_dec::encode::encode;
pub use crate::enc_dec::decode::decode;
pub use crate::enc_dec::encode_to_vec::encode_to_vec;
pub use crate::enc_dec::decode_from_slice::decode_from_slice;




