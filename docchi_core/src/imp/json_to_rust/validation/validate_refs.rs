//use crate::HashM;
use crate::error::CoreResult;
use crate::imp::json_to_rust::names::Names;
use crate::imp::rust_to_json::name_with_suffix::name_with_suffix;
use crate::imp::structs::ref_def_obj::RefDefObj;
use crate::imp::structs::ref_value::RefSabValue;
use crate::imp::structs::qv::Qv;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::root_value::RootValue;
use crate::imp::structs::util::hash_m::HashM;

pub(crate) fn validate_refs(def : &RefDefObj, sabun : &HashM<String, RefSabValue>, root : &RootObject, can_use_old: bool, names : &Names) -> CoreResult<()>{
    if def.is_enum(){
       if sabun.len() != 1{
           Err(format!("{} one of the Enum's member must be defined", names))?
       }
    } else{
        //defのdefault値がemptyで、sabunでも定義されていないのを探す。もっと効率的に実装できないものかなあ
        for (name, _, def_val) in def.refs(){
            if sabun.contains_key(name) == false{
                if let Qv::Val(v) = def_val.value(){
                    if v.is_empty(){
                        Err(format!("{} ref {} is not defined", names, name))?
                    }
                }

            }
        }
    }

    for (name, sab_val) in sabun{
        if can_use_old == false && def.old().contains(name) {
            Err(format!("{} {} is old", names, name))?
        }
        match def.refs().get(name){
            Some(h) =>{
                if h.acceptable(sab_val) == false{
                    Err(format!("{} {} {} is not valid for {}", names, name, sab_val.value().js_string(), name_with_suffix(name, h.var_type())))?
                }
                match sab_val.value() {
                    Qv::Val(id) =>{
                        if id.is_empty(){
                            Err(format!("{} ref {} is empty", names, name))?
                        }
                        match root.default().get(name) {
                            Some(RootValue::Table(d)) => {
                                if d.list().get(id).is_none() {
                                    Err(format!("{}'s {} was not found {}", name, id, names))?
                                }
                                if can_use_old == false && d.old().contains(id) {
                                    Err(format!("{}'s {} is old {}", name, id, names))?
                                }
                                continue;
                            },
                            Some(_) => {
                                Err(format!("{} the root object's {} was not Data", names, name))?
                            },
                            None => {
                                Err(format!("{} the root object's {} was not found", names, name))?
                            }
                        }
                    },
                    _ =>{
                        //nullとかundefinedとかは有効な値である
                    }
                }
            },
            None =>{ Err(format!("{} there's no default ref members named {}", names, name))? }
        }
    }


    return Ok(());
}