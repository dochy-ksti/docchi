use crate::imp::rust_to_json::list::tmp_json_list::TmpJsonRefs;
use crate::imp::rust_to_json::list::ref_def_obj_to_json::reconstruct_ref_value;
use std::collections::BTreeMap;
use crate::imp::rust_to_json::string_set_to_json::string_set_to_json_short;
use crate::imp::structs::my_json::Value;

pub(crate) fn tmp_refs_to_json(refs : &TmpJsonRefs) -> Value{
    let mut result :BTreeMap<String, Value> = BTreeMap::new();
    for (name, val) in &refs.map{
        let (name, val) = reconstruct_ref_value(name, val.value(), val.var_type());
        result.insert(name, val);
    }

    if let Some(old) = &refs.old{
        result.insert("Old".to_string(), string_set_to_json_short(old));
    }

    return Value::Map(result);
}