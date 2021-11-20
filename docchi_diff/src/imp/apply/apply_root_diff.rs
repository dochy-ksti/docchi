use docchi_core::structs::{RootObject, MetaValue, RootSabValue};
use crate::imp::structs_read::RootDiffR;
use crate::imp::apply::apply_list_diff::apply_list_diff;
use crate::diff_error::DiffError;
use crate::imp::apply::diff_to_new_list::diff_to_new_list;

pub(crate) fn apply_root_diff(root : &mut RootObject, diff : RootDiffR) -> Result<(), DiffError>{
    let (params, lists) = diff.deconstruct();
    let (_default, sabun, meta)
        = root.def_and_mut_sab();
    //let (mut default, mut sabun, old, meta) = root.deconstruct();
    for (id, p) in params{
        let (key, _meta_val) = if let Some(v) = meta.get(id){ v } else{
            unreachable!("meta is not valid apply_root_diff")
        };
        sabun.insert(key.to_string(), RootSabValue::Param(p));
    }
    for (id, list_diff) in lists{
        let (key, meta_val) = if let Some(v) = meta.get(id){ v } else{
            unreachable!("list meta is not valid apply_root_diff")
        };
        // let v = if let Some(v) = default.get(key){ v } else{
        //     unreachable!("invalid default apply_root_diff")
        // };
        match meta_val{
            MetaValue::MList(tables) | MetaValue::OptMil(tables) =>{
                if let Some(RootSabValue::Mut(m)) = sabun.get_mut(key){
                    if let Some(m) = m{
                        if let Some(list_diff) = list_diff{
                            apply_list_diff(m.list_mut(), list_diff, tables)?;
                        } else{
                            sabun.insert(key.to_string(), RootSabValue::Mut(None));
                        }
                    } else{
                        if let Some(list_diff) = list_diff{
                            sabun.insert(key.to_string(),
                                         RootSabValue::Mut(Some(diff_to_new_list(list_diff, tables)?)));
                        } else{

                        }
                    }
                } else{
                    unreachable!("invalid RootSabValue apply_root_diff")
                }
            },
            _ =>{
                unreachable!("invalid meta apply_root_diff")
            }
        }
    }
    Ok(())
}

// pub(crate ) fn apply_root_diff(root : RootObject, diff : RootDiffR) -> Result<RootObject, DiffError>{
//     let (params, lists) = diff.deconstruct();
//     let (mut default, mut sabun, old, meta) = root.deconstruct();
//     for (id, p) in params{
//         let (key, _meta_val) = if let Some(v) = meta.get(id){ v } else{
//             unreachable!("meta is not valid apply_root_diff")
//         };
//         sabun.insert(key.to_string(), p);
//     }
//     for (id, list_diff) in lists{
//         let (key, meta_val) = if let Some(v) = meta.get(id){ v } else{
//             unreachable!("list meta is not valid apply_root_diff")
//         };
//         let (_id, v) = if let Some(v) = default.get_mut(key){ v } else{
//             unreachable!("invalid default apply_root_diff")
//         };
//         let tables = if let MetaValue::MList(tables) = meta_val{ tables } else{
//             unreachable!("invalid meta apply_root_diff")
//         };
//         match v{
//             RootValue::MList(m) =>{ apply_list_diff(m.list_mut(), list_diff.unwrap(), tables)? },
//             _ =>{ unreachable!("invalid RootType apply_root_diff") },
//         }
//     }
//     Ok(RootObject::construct(default, sabun, old, meta))
// }