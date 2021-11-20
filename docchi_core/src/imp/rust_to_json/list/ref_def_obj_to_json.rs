use std::collections::BTreeMap;
use crate::imp::structs::ref_def_obj::RefDefObj;
use crate::imp::structs::qv::Qv;
use crate::imp::structs::var_type::VarType;
use crate::imp::structs::my_json::Value;

pub(crate) fn ref_def_obj_to_json(obj : &RefDefObj) -> BTreeMap<String, Value>{
    let mut result : BTreeMap<String, Value> = BTreeMap::new();

    for (key,_, value) in obj.refs() {
        let (key, value) = reconstruct_ref_value(key, value.value(), value.var_type());
        result.insert(key, value);
    }

    return result;
}

pub(crate) fn reconstruct_ref_value(name : &String, value : &Qv<String>, value_type : VarType) -> (String, Value){
    (format!("{}{}", name.to_string(), value_type.to_suffix()), match value{
        Qv::Val(v) => Value::String(v.to_string()),
        Qv::Null => Value::Null,
        Qv::Undefined => Value::Undefined,
    })
}
