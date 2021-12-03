pub(crate) mod names;
pub(crate) mod json_obj_to_rust;
pub(crate) mod json_name;
pub(crate) mod tmp;
pub(crate) mod json_item_to_rust;
pub(crate) mod json_array_to_rust;
pub(crate) mod array_null;
pub(crate) mod list;
pub(crate) mod get_old;
pub(crate) mod get_id;
pub(crate) mod get_refs;
pub(crate) mod construct_root;
pub(crate) mod validation;
pub(crate) mod set_empty_mils;
pub(crate) mod roots;

use docchi_json5::JVal;
use names::Names;
use crate::error::CoreResult;
use crate::imp::structs::rust_value::RustValue;
use crate::imp::structs::var_type::VarType;
use crate::imp::structs::root_obj::RootObject;

pub(crate) fn json_root_to_rust(json : &str) -> CoreResult<RootObject>{
    let jval = docchi_json5::from_str(json)?;

    return match jval{
        JVal::Map(map, span) =>{
            let tmp = json_obj_to_rust::json_obj_to_rust(&map, false, &span, &Names::new(""))?;
            Ok(tmp.into_root_obj()?)
        },
        _ =>{
            Err(format!(r#"root object is not found"#))?
        }
    };
}

pub(crate) fn json_item_str_to_rust(json : &str, item_name : &str, var_type : VarType) -> CoreResult<RustValue>{
    let jval = docchi_json5::from_str(json)?;

    json_item_to_rust::json_item_to_rust(item_name, var_type, &jval, &Names::new(""))
}
