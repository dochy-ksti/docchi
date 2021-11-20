use crate::imp::json_to_rust::tmp::tmp_obj::{ IdValue};
//use crate::{HashM};
use std::collections::{BTreeSet, BTreeMap};
use crate::imp::structs::rust_value::RustValue;
use crate::imp::structs::rust_list::{ConstItem, MutItem, ConstTable, ConstListVal, MutListVal, ConstList};
use crate::imp::structs::ref_value::RefValue;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::util::hash_m::{HashS, HashM};
use crate::imp::structs::mut_list_def::MutListDef;

pub(crate) struct TmpJsonList{
    pub(crate) vec : Vec<TmpJsonObj>,
    pub(crate) old : Option<BTreeSet<String>>,
    pub(crate) default : Option<ListDefObj>,
    //pub(crate ) compatible : Option<BTreeSet<String>>,
    pub(crate) next_id : Option<u64>,
}

pub(crate) struct TmpJsonObj{
    pub(crate) default : BTreeMap<String, RustValue>,
    pub(crate) id : Option<IdValue>,
    pub(crate) refs: Option<TmpJsonRefs>,
    pub(crate) old : Option<BTreeSet<String>>,
}

impl TmpJsonObj{
    pub(crate) fn from_list_item(l : &ConstItem, id : Option<&String>) -> TmpJsonObj{
        let value_map : HashM<String, RustValue> = l.values().iter().map(|(k,v)| (k.to_string(), v.clone().into_rust_value_for_json())).collect();
        let ref_map : HashM<String, RefValue> = l.refs().iter().map(|(k,v)| (k.to_string(), v.clone().into_ref_value_for_json())).collect();
        TmpJsonObj{ default : btree_map(&value_map),
            refs : TmpJsonRefs::from_list_item(&ref_map),
            id : id.map(|s| IdValue::Str(s.to_string())), old : None }
    }

    pub(crate) fn from_mut_list_item(l : &MutItem, id : u64) -> TmpJsonObj{
        let value_map : HashM<String, RustValue> = l.values().iter().map(|(k,v)| (k.to_string(), v.clone().into_rust_value_for_json())).collect();
        let ref_map : HashM<String, RefValue> = l.refs().iter().map(|(k,v)| (k.to_string(), v.clone().into_ref_value_for_json())).collect();
        TmpJsonObj{
            default : btree_map(&value_map),
            refs : TmpJsonRefs::from_list_item(&ref_map),
            id : Some(IdValue::Num(id)), old : None }
    }
}

pub(crate) struct TmpJsonRefs{
    pub(crate) map : BTreeMap<String, RefValue>,
    pub(crate) old : Option<BTreeSet<String>>,
    pub(crate) is_enum : bool,
}

impl TmpJsonRefs{
    pub(crate) fn from_map(map : &HashM<String, RefValue>, old : Option<&HashS<String>>, is_enum : bool) -> TmpJsonRefs{
        TmpJsonRefs{
            map : btree_map(map),
            old : old.map(|s| btree_set(s)), is_enum }
    }

    pub(crate) fn from_list_item(map : &HashM<String, RefValue>) -> Option<TmpJsonRefs> {
        if map.len() != 0 {
            Some(TmpJsonRefs::from_map(map, None, false))
        } else { None }
    }
}

fn get_from_set(set : &HashS<String>) -> Option<BTreeSet<String>>{
    if set.is_empty(){
        None
    } else{
        Some(btree_set(set))
    }
}

impl TmpJsonList{
    pub(crate) fn from_const_data(l : &ConstTable) -> TmpJsonList{
        //最近のハッシュマップは中身の順番がランダム化されるようなので、btree_mapにうつして順番を揃える
         TmpJsonList{ vec: btree_map(l.list()).iter().map(|(id,item)| TmpJsonObj::from_list_item(item, Some(id))).collect(),
             next_id: None, old : get_from_set(l.old()), default : Some(l.default().clone()) }
    }

    pub(crate) fn from_const_list(l : &ConstList) -> TmpJsonList{
        TmpJsonList{ vec: l.list().iter().map(|item| TmpJsonObj::from_list_item(item, None)).collect(),
            next_id: None, old : None, default : Some(l.default().clone()) }
    }

    pub(crate) fn from_mut_list(d : &MutListDef, l : &Option<MutListVal>) -> TmpJsonList{
        let (vec, next_id) = if let Some(l) = l{
            (l.list().iter().map(|(id,item)| TmpJsonObj::from_mut_list_item(item, *id)).collect(),
            l.list().next_id())
        } else{
            //undefinedは勝手に空リストにしてしまう
            (vec![], 0)
        };
        TmpJsonList{ vec,
            next_id: Some(next_id), old : None, default : Some(d.default().clone()) }
    }

    // pub(crate ) fn from_inner_data(l : &InnerData) -> TmpJsonList{
    //     TmpJsonList{ vec : btree_map(l.list()).iter().map(|(id, item)| TmpJsonObj::from_list_item(item, Some(id))).collect(),
    //         compatible : None, next_id: None, old : get_from_set(l.old()), default : None }
    // }

    pub(crate) fn from_inner_list(l : &ConstListVal) -> TmpJsonList{
        TmpJsonList{ vec : l.list().iter().map(|item| TmpJsonObj::from_list_item(item, None)).collect(),
            next_id: None, old : None, default : None }
    }

    pub(crate) fn from_inner_mut(l : &MutListVal) -> TmpJsonList{
        TmpJsonList{ vec : l.list().iter().map(|(id, item)| TmpJsonObj::from_mut_list_item(item, *id)).collect(),
            next_id: None, old : None, default : None }
    }
}

pub(crate) fn btree_set(hash : &HashS<String>) -> BTreeSet<String>{
    hash.iter().map(|s| s.to_string()).collect()
}

pub(crate) fn btree_map<T : Clone>(hash : &HashM<String, T>) -> BTreeMap<String, T>{
    hash.iter().map(|(key,val)|(key.to_string(), val.clone())).collect()
}
