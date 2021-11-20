use docchi_core::structs::{MutItem, ListDefObj, RustParam, ListSabValue, ListDefValue, Qv, MetaTables};
use crate::imp::structs_write::{ListItemDiffW, ListDiffW};
use std::collections::BTreeMap;
use crate::imp::prepare::get_mlist_diff::get_mlist_diff;
use crate::imp::prepare::new_list::new_list;
use docchi_core::IdentityEqual;

pub(crate) fn compare_items<'a, 'b>(from : &'a MutItem, to : &'b MutItem, def : &'b ListDefObj, meta : &'b MetaTables) -> Option<ListItemDiffW<'b>>{
    let ref_def = def.refs();
    let def = def.default();
    let mut params : BTreeMap<usize, &RustParam> = BTreeMap::new();
    let mut lists : BTreeMap<usize, Option<ListDiffW>> = BTreeMap::new();

    let from_values = from.values();
    for (key, value) in to.values(){
        match value{
            ListSabValue::Param(p) =>{
                if let Some((id, _item)) = def.get_with_id(key) {
                    if let Some(ListSabValue::Param(from_p)) = from_values.get(key) {
                        if p.identity_eq(from_p) == false {
                            params.insert(id, p);
                        }
                    } else {
                        params.insert(id, p);
                    }
                }
            },
            ListSabValue::Mil(mil) =>{
                if let Some((id, ListDefValue::MilDef(mil_def))) = def.get_with_id(key) {
                    let meta = if let Some(tables) = meta.items().get_tables(id) { tables } else {
                        panic!("invalid meta")
                    };
                    if let Some(ListSabValue::Mil(from_mil)) = from_values.get(key) {
                        if from_mil.is_none() && mil.is_none() {
                            continue;
                        }
                        if let Some(from_mil) = from_mil {
                            if let Some(mil) = mil {
                                if let Some(diff) = get_mlist_diff(from_mil.list(), mil.list(), mil_def.default(), meta) {
                                    lists.insert(id, Some(diff));
                                }
                            } else{
                                //コンバート時にundefinedになりうる
                                lists.insert(id, None);
                            }
                        } else{
                            if let Some(mil) = mil {
                                lists.insert(id, Some(new_list(mil,
                                                               mil_def.default(), meta)));
                            }
                        }
                    } else{
                        if let Some(mil) = mil {
                            lists.insert(id, Some(new_list(mil,
                                                           mil_def.default(), meta)));
                        } else{
                            lists.insert(id, None);
                        }
                    }
                }
            },
            ListSabValue::Cil(_) =>{}
        }
    }

    let mut refs : BTreeMap<usize, &Qv<String>> = BTreeMap::new();
    let from_ref = from.refs();
    for (key, value) in to.refs(){
        if let Some((id, _v)) = ref_def.refs().get_with_id(key) {
            let to_val = value.value();
            if let Some(from_value) = from_ref.get(key) {
                if from_value.value() != to_val {
                    refs.insert(id, to_val);
                }
            } else{
                refs.insert(id, to_val);
            }
        }
    }

    if params.len() == 0 && refs.len() == 0 && lists.len() == 0{
        None
    } else {
        Some(ListItemDiffW::new(params, refs, lists))
    }
}