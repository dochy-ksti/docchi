use docchi_json5::jval::JVal;
use super::names::Names;
use super::json_name::{json_name, NameType, SystemNames};
use super::json_item_to_rust::json_item_to_rust;
use crate::error::CoreResult;

use crate::imp::json_to_rust::tmp::tmp_obj::TmpObj;
use crate::imp::json_to_rust::get_old::get_old;
use crate::imp::json_to_rust::get_id::get_id;
use crate::imp::json_to_rust::get_refs::get_ref;
use docchi_json5::jval::Span;
use crate::imp::json_to_rust::json_item_to_rust::json_item_to_rust_ref;
use linked_hash_map::LinkedHashMap;
//use linked_hash_map::LinkedHashMap;

pub(crate) fn json_obj_to_rust(v : &LinkedHashMap<String, JVal>, is_ref_obj : bool, span : &Span, names : &Names) -> CoreResult<TmpObj>{
    let mut r  = TmpObj::new(v.len(),span.clone());
    let mut counter = 0;
    for (k,v) in v{
        let id = counter;

        let name = json_name(k).ok_or_else(|| format!("{} {} is not a valid name {}",v.line_str(), k, names))?;
        match name{
            NameType::Name(name, vt) =>{
                let v = if is_ref_obj {
                    //きったないコードだなあ。でもこれが一番簡単で分かりやすいと思う・・・
                    json_item_to_rust_ref(&name, vt, v, names)?
                } else{
                    json_item_to_rust(&name, vt,v, names)?
                };

                r.insert_default(name.to_string(), id, v);
                counter += 1;
            },
            NameType::SystemName(sn) =>{
                match sn{
                    SystemNames::ID =>{
                        if r.id.is_none() {
                            if let Some(id) = get_id(v){
                                r.id = Some(id);
                            } else {
                                Err(format!("{} ID must be a string or a num : {} {}", v.line_str(), v.slice(), names))?
                            }
                        } else{
                            Err(format!("{} ID is defined multiple times {}", v.line_str(), names))?;
                        }
                    },
                    SystemNames::Ref | SystemNames::Enum =>{
                        if r.refs.map.len() == 0{
                            match &v {
                                JVal::Map(map, span) =>{
                                    let mut refs = get_ref(map, span,names)?;
                                    match sn{
                                        SystemNames::Enum =>{ refs.is_enum = true; }
                                        _=>{},
                                    }
                                    r.refs = refs;
                                },
                                _ =>{ Err(format!("{} Ref must be an object {}", v.line_str(), names))?;}
                            }
                        } else {
                            Err(format!("{} (Ref|Enum) is defined multiple times {}", v.line_str(), names))?;
                        }
                    },
                    SystemNames::Old => {
                        match &v {
                            JVal::Array(a, _span) => {
                                r.old = get_old(a, names)?;
                            },
                            _ => { Err(format!("{}  {}", v.line_str(), names))?; }
                        }
                    }
                }
            }
        }

    }
    Ok(r)
}