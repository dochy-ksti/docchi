use crate::imp::json_to_rust::validation::validate_root::validate_root;
use crate::imp::version_adjuster::adjust_mut_list::adjust_mut_list;
//use crate::{HashM, HashMt};
use crate::error::CoreResult;
use crate::imp::json_to_rust::names::Names;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::root_value::RootValue;
use crate::imp::structs::root_sab_value::RootSabValue;
use std::sync::Arc;


// paramのsabunがあれば上書き、mut_listはoldのものを全部入れ、（あるなら）newの方のものは全削除して入れ替える
// 基本的に、新バージョンのjsonと旧バージョンのデータが有り、旧バージョンのデータはRootのsabunとMutListには変更が加えられているだろう
// Defaultが更新されるので、undefinedが設定される。

/// Adjust old data to be compatible with the new version.
/// The strategy of the adjustment is described in the manual.
pub fn adjust_versions(new : RootObject, old : RootObject, validation : bool) -> CoreResult<RootObject>{

    let (def, mut sabun_v, old_hash, meta) = new.deconstruct();

    let sabun = Arc::make_mut(&mut sabun_v);
    //let mut new_map :HashM<String, (usize, RootValue)> = HashMt::with_capacity(def.len());

    let (old_def,mut old_sabun, _, _) = old.deconstruct();

    let old_sabun = Arc::make_mut(&mut old_sabun);

    for (def_key, (_id, def_value)) in def.def(){
        match def_value{
            RootValue::Param(p,v) =>{
                let undef = if v.undefiable(){
                    if old_def.contains_key(def_key) == false{
                        sabun.insert(def_key.to_string(),RootSabValue::Param(p.to_undefined()));
                        true
                    } else {
                        false
                    }
                } else{
                    false
                };

                if undef == false {
                    if let Some(param) = old_sabun.remove(def_key) {
                        sabun.insert(def_key.to_string(), param);
                    }
                }
            },
            RootValue::MList(m) =>{
                let undef = if m.undefiable(){
                    if old_def.contains_key(def_key) == false{
                        sabun.insert(def_key.to_string(),RootSabValue::Mut(None));
                        true
                    } else {
                        false
                    }
                } else{
                    false
                };

                if undef == false {
                    if let Some(RootSabValue::Mut(m_val)) = old_sabun.remove(def_key) {
                        if let Some(m_val) = m_val {
                            let new_m = adjust_mut_list(m.default(), m_val, &Names::new(def_key))?;
                            sabun.insert(def_key.to_string(),RootSabValue::Mut(Some(new_m)));
                        } else{
                            sabun.insert(def_key.to_string(),RootSabValue::Mut(None));
                        }
                    }
                }
            },
            _ =>{
                //MutとParam以外にadjustする対象はないはず
            },
        }
    }

    let new = RootObject::construct(def, sabun_v, old_hash, meta);

    if validation{
        validate_root(&new, true)?
    }
    return Ok(new);
}

