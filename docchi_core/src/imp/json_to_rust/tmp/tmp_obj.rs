//use crate::{HashM, HashMt};
use docchi_json5::Span;
use crate::error::CoreResult;
use crate::imp::structs::rust_value::{RustValue};
use crate::imp::structs::ref_value::{RefValue, RefSabValue};
use crate::imp::structs::ref_def_obj::{RefDefObj};
use crate::imp::structs::rust_list::{ConstItem, MutItem};
use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::root_value::RootValue;
use crate::imp::structs::list_value::{ ListDefValue};
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::util::hash_m::{HashS, HashSt, HashM};
use crate::HashMt;
use crate::imp::structs::list_sab_value::ListSabValue;
use crate::imp::structs::root_sab_value::RootSabValue;
use crate::imp::structs::root_def_obj::RootDefObj;

pub(crate) struct TmpObj{
    pub(crate) default : HashM<String, (usize, RustValue)>,
    pub(crate) id : Option<IdValue>,
    pub(crate) include : Vec<String>,
    pub(crate) refs: TmpRefs,
    pub(crate) old : HashS<String>,
    pub(crate) span : Span,
}

pub(crate) struct TmpRefs{
    pub(crate) map : HashM<String, (usize, RefValue)>,
    pub(crate) old : HashS<String>,
    pub(crate) is_enum : bool,
    pub(crate) span : Span,
}

impl TmpRefs{
    pub(crate) fn new(capacity : usize, span : Span) -> TmpRefs{
        TmpRefs{ map : HashMt::with_capacity(capacity), old : HashSt::new(), is_enum : false, span }
    }

    // pub fn get_hash_map(self) -> HashM<String, RefValue>{
    //     self.map.into_iter().collect()
    // }

    pub(crate) fn into_ref_def_obj(self) -> RefDefObj{
        RefDefObj::new(self.map,  self.is_enum, self.old)
    }
}


pub(crate) enum IdValue{
    Str(String),
    Num(u64)
}

impl TmpObj{
    pub(crate) fn new(capacity : usize, span : Span) -> TmpObj{
        TmpObj{ default : HashMt::with_capacity(capacity), id : None, include : vec![], refs : TmpRefs::new(0,span.clone()), old : HashSt::new(), span }
    }

    pub(crate) fn into_root_obj(self) -> CoreResult<RootObject>{
        fn to_root_hash(map : HashM<String, (usize, RustValue)>) ->
                                                                 CoreResult<(
                                                                     HashM<String, (usize, RootValue)>,
                                                                     HashM<String, RootSabValue>)>{
            let mut result : HashM<String, (usize, RootValue)> = HashMt::with_capacity(map.len());
            let mut sabun : HashM<String, RootSabValue> = HashMt::new();

            for (key, (id, value)) in map{
                match value.into_root_value(){
                    Ok((def, val)) =>{
                        result.insert(key.to_string(), (id, def));
                        if let Some(val) = val{
                            sabun.insert(key, val);
                        }
                    },
                    Err(type_s) => Err(format!("{} root object can't have {}", key, type_s))?,
                }
            }
            Ok((result, sabun))
        }

        if self.id.is_some(){
            Err(format!("ID is not needed for the root obj"))?
        }
        if self.refs.map.len() != 0{
            Err(format!("Ref is not needed for the root obj"))?
        }

        let (def, sabun) = to_root_hash(self.default)?;
        Ok(RootObject::new(RootDefObj::new(def), sabun, self.old))
    }

    pub(crate) fn into_list_def_obj(self) -> CoreResult<ListDefObj>{
        if self.id.is_some(){
            Err(format!("{} list def can't have ID {}", self.span.line_str(), self.span.slice()))?
        }
        Ok(ListDefObj::new(to_list_def_val_map(self.default, &self.span)?,
                           self.refs.into_ref_def_obj(), self.old))
    }

    pub(crate) fn insert_default(&mut self, s : String, id : usize, v : RustValue){
        self.default.insert(s, (id,v));
    }



    pub(crate) fn into_mut_list_item(self, id : usize) -> CoreResult<(u64, MutItem)>{
        let id = match self.id {
            Some(IdValue::Num(id)) => id,
            Some(_) =>{
                Err(format!("{} Mut List's item's ID must be a number {}", self.span.line_str(), self.span.slice()))?
            },
            None => id as u64,
        };

        if self.old.len() != 0{
            Err(format!("{} Old is not needed for a mut item {}", self.span.line_str(), self.span.slice()))?
        }
        if self.refs.old.len() != 0{
            Err(format!("{} Old is not needed for a mut item {}", self.refs.span.line_str(), self.refs.span.slice()))?
        }

        Ok((id, MutItem::new(to_list_sab_map(self.default, &self.span)?, to_ref_sab_map(self.refs.map))))
    }
    pub(crate) fn into_list_item(self) -> CoreResult<ConstItem>{
        if self.id.is_some(){
            Err(format!("{} ID is not needed for a list item {}", self.span.line_str(), self.span.slice()))?
        }
        if self.old.len() != 0{
            Err(format!("{} Old is not needed for a list item {}", self.span.line_str(), self.span.slice()))?
        }
        if self.refs.old.len() != 0{
            Err(format!("{} Old is not needed for a list item {}", self.refs.span.line_str(), self.refs.span.slice()))?
        }
        Ok(ConstItem::new(to_list_sab_map(self.default, &self.span)?, to_ref_sab_map(self.refs.map)))
    }

    pub(crate) fn into_list_item_with_id(self) -> CoreResult<(String, ConstItem)>{
        if self.id.is_none(){
            Err(format!("{} ID must be defined {}", self.span.line_str(), self.span.slice()))?
        }
        if self.old.len() != 0{
            Err(format!("{} Old is not needed for a list item {}", self.span.line_str(), self.span.slice()))?
        }
        if self.refs.old.len() != 0{
            Err(format!("{} Old is not needed for a list item {}", self.refs.span.line_str(), self.refs.span.slice()))?
        }
        match self.id.unwrap(){
            IdValue::Str(s) =>{
                Ok((s, ConstItem::new(to_list_sab_map(self.default, &self.span)?, to_ref_sab_map(self.refs.map))))
            },
            IdValue::Num(_) =>{
                Err(format!("{} ID must be a string {}", self.span.line_str(), self.span.slice()))?
            }
        }
    }
}

fn to_list_sab_map(map : HashM<String, (usize, RustValue)>, span : &Span) -> CoreResult<HashM<String, ListSabValue>>{
    let mut result : HashM<String, ListSabValue> = HashMt::with_capacity(map.len());
    for (k,(_, v)) in map{
        let sab = match v.into_list_sab_value(){
            Ok(a) => a,
            Err(s) =>{
                Err(format!("{} {} list items can't have {}", span.line_str(), k, s))?
            }
        };
        result.insert(k, sab);
    }
    Ok(result)
}

fn to_list_def_val_map(map : HashM<String, (usize, RustValue)>, span : &Span) -> CoreResult<HashM<String, (usize, ListDefValue)>>{
    let mut result : HashM<String, (usize, ListDefValue)> = HashMt::with_capacity(map.len());
    for (idx, (k,(_id, v))) in map.into_iter().enumerate(){
        let sab = match v.into_list_def_value(){
            Ok(a) => (idx, a),
            Err(s) =>{
                Err(format!("{} {} list def can't have {}", span.line_str(), k, s))?
            }
        };
        result.insert(k, sab);
    }
    Ok(result)
}

fn to_ref_sab_map(map : HashM<String, (usize, RefValue)>) -> HashM<String, RefSabValue>{
    let mut result : HashM<String, RefSabValue> = HashMt::with_capacity(map.len());
    for(k,(_,v)) in map{
        result.insert(k, v.into_sab_value());
    }
    return result;
}



