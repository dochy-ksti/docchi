use std::collections::{BTreeMap};
use crate::imp::rust_to_json::rust_value_to_json_value::rust_value_to_json_value;
use crate::imp::structs::rust_value::RustValue;
use crate::imp::structs::my_json::Value;

pub(crate) fn value_map_to_json(map : &BTreeMap<String, RustValue>) -> BTreeMap<String, Value>{
    let mut result = BTreeMap::new();

    for (name,val) in map{
        let (name, val) = rust_value_to_json_value(val, name);
        result.insert(name, val);
    }

    return result;
}