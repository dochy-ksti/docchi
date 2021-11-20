use docchi_core::structs::{MutItem, ListDefObj, RustParam, ListSabValue, ListDefValue, Qv, MetaTables};
use crate::imp::structs_write::{ListItemDiffW, ListDiffW};
use std::collections::BTreeMap;
use crate::imp::prepare::new_list::new_list;

pub(crate) fn new_item<'a>(item : &'a MutItem, def : &'a ListDefObj, meta : &'a MetaTables) -> ListItemDiffW<'a>{
    let ref_def = def.refs();
    let def = def.default();
    let mut params : BTreeMap<usize, &RustParam> = BTreeMap::new();
    let mut lists : BTreeMap<usize, Option<ListDiffW>> = BTreeMap::new();
    for (key, value) in item.values(){
        match value{
            ListSabValue::Param(p) =>{
                if let Some((id, _item)) = def.get_with_id(key) {
                    params.insert(id, p);
                } else{
                    panic!("def is invalid");
                }
            },
            ListSabValue::Mil(mil) =>{
                if let Some((id, ListDefValue::MilDef(mil_def))) = def.get_with_id(key) {
                    if let Some(mil) = mil {
                        if let Some(tables) = meta.items().get_tables(id) {
                            lists.insert(id, Some(new_list(mil, mil_def.default(), tables)));
                        } else{
                            panic!("invalid meta");
                        }
                    } else{
                        //コンバート時にundefinedになりうる
                        lists.insert(id, None);
                    }
                }
            },
            ListSabValue::Cil(_) =>{}
        }
    }

    let mut refs : BTreeMap<usize, &Qv<String>> = BTreeMap::new();
    for (key, value) in item.refs(){
        if let Some((id,_v)) = ref_def.refs().get_with_id(key){
            refs.insert(id, value.value());
        }
    }

    ListItemDiffW::new(params, refs, lists)
}