use crate::error::CoreResult;
use crate::imp::rust_to_json::list::value_map_to_json::value_map_to_json;
use crate::imp::rust_to_json::list::tmp_json_list::{btree_map, btree_set};
use crate::imp::rust_to_json::string_set_to_json::{string_set_to_json_short};
use crate::{HashM, HashMt};
use crate::imp::structs::rust_value::{RustValue};
use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::my_json::Value;
use crate::imp::structs::root_value::RootValue;
use crate::imp::structs::root_sab_value::RootSabValue;

//本来デフォルト値と差分が保存されているのだが、見やすくするためにまとめてデフォルト値にしてしまう。
//デフォルト値も差分も全部Json化したいユースケースもあるかもしれない・・・？

/// Convert RootObject to JSON
/// That JSON can be converted back to RootObject, but they are not identical.
/// This may not have very useful use cases,
/// but converting data to human readable text is good
pub fn root_to_json_new_default(obj : &RootObject) -> CoreResult<Value> {
    let mut result : HashM<String,RustValue> = HashMt::with_capacity(obj.default().len());
    let default = obj.default().def().clone();
    let mut sabun = obj.sabun().clone();
    let old = obj.old().clone();

    for (name, (_id, val)) in default{
        if let RootValue::Param(p,vt) = val{
            if let Some(RootSabValue::Param(sab_param)) = sabun.remove(&name){
                result.insert(name, RustValue::Param(sab_param, vt));
            } else{
                result.insert(name, RustValue::Param(p, vt));
            }
        } else{
            if let Some(sab) = sabun.remove(&name) {
                result.insert(name, val.into_rust_value(sab)?);
            }
        }
    }

    let mut map = value_map_to_json(&btree_map(&result));
    map.insert( "Old".to_string(), string_set_to_json_short(&btree_set(&old)));

    return Ok(Value::Map(map));
}

