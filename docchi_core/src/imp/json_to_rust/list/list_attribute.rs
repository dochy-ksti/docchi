use docchi_json5::{JVal, Span};
use super::super::names::Names;
use super::get_default::get_default;
use crate::error::CoreResult;
use crate::imp::json_to_rust::get_old::get_old;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::util::hash_m::HashS;

pub(crate) enum ListAttribute{
    Default(ListDefObj),
    Old(HashS<String>),
    //Compatible(HashS<String>),
    NextID(u64),
}


pub(crate) fn list_attribute(array : &Vec<JVal>, span : &Span, names : &Names) -> CoreResult<ListAttribute>{
    let error_message = "List's array must be Default, Old, Compatible or NextID";

    if array.len() == 0{
        Err(format!("{} {} {} {}", span.line_str(), span.slice(), error_message, names))?
    }
    return match &array[0]{
        JVal::String(s, _) =>{
            match s.as_str(){
                "Default" =>{
                    let def = get_default(&array[1..], span, names)?;
                    Ok(ListAttribute::Default(def))
                },
                "Old" =>{
                    let old = get_old(&array[1..], names)?;
                    Ok(ListAttribute::Old(old))
                },
                // "Compatible" =>{
                //     let compatible = get_compatible(&array[1..], names)?;
                //     Ok(ListAttribute::Compatible(compatible))
                // },
                "NextID" =>{
                    if array.len() == 2{
                        match array[1]{
                            JVal::Int(n, _) =>{
                                return Ok(ListAttribute::NextID(n as u64))
                            }
                            _ =>{}
                        }
                    }
                    Err(format!(r#"{} {} NextID must be ["NextID", num] {}"#, span.line_str(), span.slice(), names))?
                }
                _ =>{
                    Err(format!("{} {} {} {}", span.line_str(), span.slice(), error_message, names))?
                }
            }
        },
        //タグなしでいきなりデフォルトを書く省略記法
        JVal::Map(_,_) =>{
            let def = get_default(&array, span, names)?;
            Ok(ListAttribute::Default(def))
        }
        _ =>{
            Err(format!("{} {} {} {}", span.line_str(), span.slice(), error_message, names))?
        }
    };
}