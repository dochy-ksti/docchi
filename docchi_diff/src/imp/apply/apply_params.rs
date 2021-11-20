use docchi_core::HashM;
use docchi_core::structs::{ListSabValue, MetaTable, RustParam};
use crate::diff_error::DiffError;

pub(crate) fn apply_params(params : Vec<(usize, RustParam)>, meta : &MetaTable,
                    r : &mut HashM<String, ListSabValue>) -> Result<(), DiffError>{

    for (id,param)in params{
        let (key, _) = if let Some(v) = meta.get(id){ v } else{
            Err("meta is invalid apply_list_diff:new_item")?
        };
        r.insert(key.to_string(), ListSabValue::Param(param));
    }
    return Ok(());
}