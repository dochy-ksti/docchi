use crate::imp::rust_to_json::list::tmp_json_list::TmpJsonObj;
use crate::imp::rust_to_json::list::value_map_to_json::value_map_to_json;
use crate::imp::json_to_rust::tmp::tmp_obj::IdValue;
use crate::imp::rust_to_json::string_set_to_json::string_set_to_json_short;
use crate::imp::rust_to_json::list::tmp_refs_to_json::tmp_refs_to_json;
use crate::imp::structs::my_json::Value;

pub(crate) fn tmp_obj_to_json(obj : &TmpJsonObj) -> Value{
    let mut result = value_map_to_json(&obj.default);
    if let Some(id) = &obj.id{
        match id{
            IdValue::Num(id) =>{ result.insert("ID".to_string(), Value::Number(*id as f64)); },
            IdValue::Str(s) =>{ result.insert("ID".to_string(), Value::String(s.to_string())); },
        }
    }
    if let Some(old) = &obj.old{
        result.insert("Old".to_string(), string_set_to_json_short(old));
    }
    if let Some(refs) = &obj.refs{
        if refs.is_enum{
            result.insert("Enum".to_string(), tmp_refs_to_json(refs));
        } else{
            result.insert("Ref".to_string(), tmp_refs_to_json(refs));
        }
    }

    Value::Map(result)
}