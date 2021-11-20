use docchi_core::structs::RootObject;
use crate::imp::write::write_root::write_root;
use crate::imp::prepare::get_root_diff::get_root_diff;
use docchi_compaction::kval_enum::KVal;
use docchi_compaction::enc_dec::encode_to_vec::encode_to_vec;
use crate::DiffResult;

/// Gets the difference from "from" to "to" in binary format.
///
/// When the diff binary is applied to a object with identical state of "from" object,
/// an object with identical state of "to" object will be constructed.
///
/// You need two Rootobjects which has the same type data,
/// which means those two are created from or adjusted to the same source file.
/// No type-checks are available so be careful.
///
/// if 'to' is not derived from 'from', the diff binary can be invalid.
///
/// This system can't restore 'unmodified' state.
/// To apply diff from newer data with a modified variable
/// to older data with the variable unmodified,
/// the system needs to restore 'unmodified' state, but it can't due to technical difficulties.
///
/// So you must make sure the "to" object is derived from "from" object. No checks are available about it too.
pub fn get_diff(from : &RootObject, to : &RootObject) -> DiffResult<Vec<u8>> {
    let kvals = get_kvals(from, to)?;
    Ok(encode_to_vec(&kvals)?)
}

pub fn get_kvals(from: &RootObject, to: &RootObject) -> DiffResult<Vec<KVal>> {
    let root_diff = get_root_diff(from, to)?;
    let kvals = write_root(root_diff)?;
    Ok(kvals)
}



