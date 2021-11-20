use docchi_core::structs::{RootObject, RootValue, RustParam, MetaValue, RootSabValue};
use std::collections::BTreeMap;
use crate::imp::structs_write::{RootDiffW, ListDiffW};
use crate::diff_error::DiffError;
use crate::imp::prepare::get_mlist_diff::get_mlist_diff;
use docchi_core::IdentityEqual;
use crate::imp::prepare::new_list::new_list;

pub(crate) fn get_root_diff<'a, 'b>(from : &'a RootObject, to : &'b RootObject) -> Result<RootDiffW<'b>, DiffError>{
    let f = from.sabun();
    let t = to.sabun();
    let def = to.default().def();

    let mut params : BTreeMap<usize, &RustParam> = BTreeMap::new();
    let mut lists : BTreeMap<usize, Option<ListDiffW>> = BTreeMap::new();

    let meta = to.meta_table();


    //デフォルト値に書き換えられた場合もsabunからremoveしたりはしないので、
    //書き換えられたことがあるものは必ずsabunにメンバがある。
    //なので変化の可能性があるものは全部toを調べればわかる
    for (key,to_val) in t {
        match to_val {
            RootSabValue::Param(p) => {
                if let Some(RootSabValue::Param(from_p)) = f.get(key) {
                    if from_p.identity_eq(p) == false {
                        if let Some((id, _v)) = def.get(key) {
                            params.insert(*id, p);
                        } else {
                            panic!("invalid def")
                        }
                    }
                } else {
                    if let Some((id, _v)) = def.get(key) {
                        params.insert(*id, p);
                    } else {
                        panic!("invalid def")
                    }
                }
            },
            RootSabValue::Mut(to_list) => {
                //RootSabValue::Mutの値は必ず存在する。Noneかもしれないが、Noneという値がある。
                let from_list = if let Some(RootSabValue::Mut(from_list)) = f.get(key) { from_list } else {
                    panic!("invalid RootSab")
                };
                if from_list.is_none() && to_list.is_none() {
                    continue;
                }
                let (&id, mdef) = if let Some((id, RootValue::MList(mdef))) = def.get(key) { (id, mdef) } else {
                    panic!("invalid def");
                };
                let meta = if let Some((_key, MetaValue::MList(meta))) = meta.get(id) { meta } else {
                    panic!("invalid meta");
                };

                if let Some(from_list) = from_list {
                    if let Some(to_list) = to_list {
                        if let Some(diff) = get_mlist_diff(from_list.list(), to_list.list(), mdef.default(), meta) {
                            lists.insert(id, Some(diff));
                        }
                    } else {
                        //コンバート時にundefinedになりうる
                        lists.insert(id, None);
                    }
                } else {
                    if let Some(to_list) = to_list {
                        lists.insert(id, Some(new_list(to_list,
                                                        mdef.default(), meta)));
                    }
                }
            },
        }
    }


    Ok(RootDiffW::new(params, lists, meta))
}