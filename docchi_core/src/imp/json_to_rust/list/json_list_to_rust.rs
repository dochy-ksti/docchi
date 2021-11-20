use docchi_json5::{JVal, Span};
use super::super::names::Names;
use crate::error::CoreResult;
use super::list_attribute::{ListAttribute, list_attribute};
use super::get_list_items::get_list_items;
use crate::imp::json_to_rust::tmp::tmp_list::TmpList;

pub(crate) fn json_list_to_rust(array : &[JVal],  span : &Span, names : &Names) -> CoreResult<TmpList> {
    let mut result = TmpList::new(array.len(),span.clone());
    for ind in 0..array.len() {
        let item = &array[ind];
        match item {
            JVal::Array(a2, span) => {
                match list_attribute(a2, span, names)?{
                    ListAttribute::Default(obj) =>{
                        if result.default.is_none() {
                            result.default = Some(obj);
                        } else{
                            Err(format!(r#"{} {} Default is defined multiple times {}"#, span.line_str(), span.slice(), names))?
                        }
                    },
                    ListAttribute::Old(old) =>{
                        if result.old.is_none() {
                            result.old = Some(old);
                        } else{
                            Err(format!(r#"{} {} Old is defined multiple times {}"#, span.line_str(), span.slice(), names))?
                        }
                    }
                    // ListAttribute::Compatible(compatible) =>{
                    //     if result.compatible.is_none() {
                    //         result.compatible = Some(compatible);
                    //     } else{
                    //         Err(format!(r#"{} {} Compatible is defined multiple times {}"#, span.line_str(), span.slice(), names))?
                    //     }
                    // },
                    ListAttribute::NextID(next_id) =>{
                        if result.next_id.is_none() {
                            result.next_id = Some(next_id);
                        } else{
                            Err(format!(r#"{} {} NextID is defined multiple times {}"#, span.line_str(), span.slice(), names))?
                        }
                    }
                }
            },
            JVal::Map(_, span) =>{

                result.vec = get_list_items(&array[ind..], span, names)?;

                break;
            },
        _ =>{ Err(format!(r#"{} {} List must consist of objects and arrays {}"#, item.span().line_str(), item.span().slice(), names))? }
        };
    }
    return Ok(result);
}






