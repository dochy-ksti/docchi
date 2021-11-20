use docchi_json5::{JVal, Span};
use crate::imp::json_to_rust::names::Names;
use crate::error::CoreResult;
use crate::imp::json_to_rust::json_obj_to_rust::json_obj_to_rust;
use crate::imp::json_to_rust::tmp::tmp_obj::TmpObj;

pub(crate) fn get_list_items(array : &[JVal],   _span : &Span, names : &Names) -> CoreResult<Vec<TmpObj>>{
    let mut result : Vec<TmpObj> = Vec::with_capacity(array.len());
    for index in 0..array.len(){
        let item = &array[index];
        match item{
            JVal::Map(map, span) =>{
                result.push(json_obj_to_rust(map, false, span, names)?)
            },
            _ =>{
                Err(format!(r#"{} List's object sequence must not be interrupted {}"#, item.span().line_str(), names))?
            }
        }
    }
    return Ok(result);
}