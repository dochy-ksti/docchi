use std::collections::{ BTreeSet};
use crate::imp::structs::my_json::Value;

///long : [tag, "hoge", "huga"] short : ["hoge", "huga"]
pub(crate) fn string_set_to_json_short(s : &BTreeSet<String>) -> Value{
    Value::Array(s.iter().map(|s| Value::String(s.to_string())).collect())
}

///[tag, "hoge", "huga"]
pub(crate) fn string_set_to_json(tag : &str, s : &BTreeSet<String>) -> Value{
    let mut vec = vec![Value::String(tag.to_string())];
    vec.extend(s.iter().map(|s| Value::String(s.to_string())));
    Value::Array(vec)
}

