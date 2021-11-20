pub mod names;
pub mod json_obj_to_rust;
pub mod json_name;
pub mod tmp;
pub mod json_item_to_rust;
pub mod json_array_to_rust;
pub mod array_null;
pub mod list;
pub mod get_old;
pub mod get_id;
pub mod get_refs;
pub mod construct_root;
pub mod validation;
pub mod set_empty_mils;
pub mod roots;

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
