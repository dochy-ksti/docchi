//use crate::{HashM, HashMt};
use crate::error::CoreResult;
use crate::imp::json_to_rust::names::Names;
use crate::imp::structs::ref_def_obj::RefDefObj;
use crate::imp::structs::ref_value::RefSabValue;
use crate::imp::structs::qv::Qv;
use crate::imp::structs::util::hash_m::{HashM, HashMt};

pub(crate) fn adjust_mut_list_item_ref(def : &RefDefObj, old_ref : &mut HashM<String, RefSabValue>, _names : &Names) -> CoreResult<HashM<String, RefSabValue>>{

    //事前に大きさが決定できないが、refのusecaseだとundefinedは少なく、default値のままが多いと思うので、sabunのlenを使う
    let mut result : HashM<String, RefSabValue> = HashMt::with_capacity(old_ref.len());

    for (def_key,_, def_v) in def.refs(){
        let sabun_v = if let Some(v) = old_ref.remove(def_key){ v } else {
            if def_v.var_type().undefiable(){
                RefSabValue::new(Qv::Undefined)
            } else{
                continue;
            }
        };
        result.insert(def_key.to_string(), sabun_v);
    }
    Ok(result)
}