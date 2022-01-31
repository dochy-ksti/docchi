use crate::imp::json_to_rust::tmp::tmp_obj::TmpObj;
use crate::{HashM, HashMt};
use docchi_json5::Span;
use crate::error::CoreResult;
use crate::imp::structs::rust_list::{ConstListVal, ConstTable, MutListVal, ConstItem, MutItem, ConstList};
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::mut_list_def::MutListDef;
use crate::imp::structs::linked_m::LinkedMap;
use crate::imp::structs::util::hash_m::{HashS, HashSt};

pub(crate) struct TmpList{
    pub(crate) vec : Vec<TmpObj>,
    ///複数回定義のエラーを検出したいのでOptionにする
    pub(crate) old : Option<HashS<String>>,
    pub(crate) default : Option<ListDefObj>,
    //pub(crate) compatible : Option<HashS<String>>,
    pub(crate) next_id : Option<u64>,
    pub(crate) span : Span,
}

impl TmpList{
    pub(crate) fn new(capacity : usize, span : Span) -> TmpList{
        TmpList{ vec : Vec::with_capacity(capacity), old : None, default : None,  next_id : None, span }
    }

    pub(crate) fn into_const_list(self) -> CoreResult<ConstList>{
        // if self.compatible.is_some(){
        //     Err(format!("{} Compatible is not needed for a List {}", self.span.line_str(), self.span.slice()))?
        // }
        if self.old.is_some(){
            Err(format!("{} Old is not needed for a List {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_none(){
            Err(format!("{} Default must be defined {}", self.span.line_str(), self.span.slice()))?
        }
        if self.next_id.is_some(){
            Err(format!("{} NextID must not be defined {}", self.span.line_str(), self.span.slice()))?
        }
        let list_item = to_list_items(self.vec)?;

        Ok(ConstList::new(self.default.unwrap(), list_item))
    }



    pub(crate) fn into_inner_list(self) -> CoreResult<ConstListVal>{
        // if self.compatible.is_some(){
        //     Err(format!("{} Compatible is not needed for InnerList {}", self.span.line_str(), self.span.slice()))?
        // }
        if self.old.is_some(){
            Err(format!("{} Old is not needed for InnerList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_some(){
            Err(format!("{} Default must not be defined for InnerList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.next_id.is_some(){
            Err(format!("{} NextID must not be defined for InnerList {}", self.span.line_str(), self.span.slice()))?
        }

        Ok(ConstListVal::new(to_list_items(self.vec)?))
    }

    pub(crate) fn into_const_data(self) -> CoreResult<ConstTable>{
        // if self.compatible.is_some(){
        //     Err(format!("{} Compatible is not needed for Data {}", self.span.line_str(), self.span.slice()))?
        // }
        if self.default.is_none(){
            Err(format!("{} Default must be defined {}", self.span.line_str(), self.span.slice()))?
        }
        if self.next_id.is_some(){
            Err(format!("{} NextID must not be defined {}", self.span.line_str(), self.span.slice()))?
        }
        let old = self.old.unwrap_or_else(|| HashSt::new());

        Ok(ConstTable::new(self.default.unwrap(), to_data_items(self.vec)?, old))
    }



    pub(crate) fn into_mut_list(self, undefiable : bool) -> CoreResult<(MutListDef, Option<MutListVal>)>{

        if self.old.is_some(){
            Err(format!("{} Old is not needed for MutList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_none(){
            Err(format!("{} Default must be defined {}", self.span.line_str(), self.span.slice()))?
        }

        let next_id = self.next_id.unwrap_or(self.vec.len() as u64);

        let items = to_mut_list_items(self.vec, next_id)?;
        //let compatible = self.compatible.unwrap_or_else(|| HashSt::new());
        let def = self.default.unwrap();
        Ok((MutListDef::new(def, undefiable), Some(MutListVal::new(items))))
    }

    pub(crate) fn into_mut_inner_list(self) -> CoreResult<MutListVal>{
        // if self.compatible.is_some(){
        //     Err(format!("{} Compatible is not needed for MutInnerList {}", self.span.line_str(), self.span.slice()))?
        // }
        if self.old.is_some(){
            Err(format!("{} Old is not needed for MutInnerList {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_some(){
            Err(format!("{} Default must not be defined {}", self.span.line_str(), self.span.slice()))?
        }

        let next_id = self.next_id.unwrap_or(self.vec.len() as u64);

        let items = to_mut_list_items(self.vec, next_id)?;

        Ok(MutListVal::new(items))
    }

    pub(crate) fn into_inner_def(self) -> CoreResult<ListDefObj>{
        // if self.compatible.is_some(){
        //     Err(format!("{} Compatible is not needed for InnerDef {}", self.span.line_str(), self.span.slice()))?
        // }
        if self.old.is_some(){
            Err(format!("{} Old is not needed for InnerDef {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_none(){
            Err(format!("{} Default must be defined {}", self.span.line_str(), self.span.slice()))?
        }
        if self.next_id.is_some(){
            Err(format!("{} NextID is not needed for InnerDef {}", self.span.line_str(), self.span.slice()))?
        }
        if self.vec.len() != 0{
            Err(format!("{} InnerDef must not have items {}", self.span.line_str(), self.span.slice()))?
        }

        Ok(self.default.unwrap())
    }

    pub(crate) fn into_inner_mut_def(self, undefinable : bool) -> CoreResult<MutListDef>{
        if self.old.is_some(){
            Err(format!("{} Old is not needed for InnerDef {}", self.span.line_str(), self.span.slice()))?
        }
        if self.default.is_none(){
            Err(format!("{} Default must be defined {}", self.span.line_str(), self.span.slice()))?
        }
        if self.next_id.is_some(){
            Err(format!("{} NextID is not needed for InnerDef {}", self.span.line_str(), self.span.slice()))?
        }
        if self.vec.len() != 0{
            Err(format!("{} InnerDef must not have items {}", self.span.line_str(), self.span.slice()))?
        }
        //let compatible = self.compatible.unwrap_or_else(|| HashSt::new());
        Ok(MutListDef::new(self.default.unwrap(), undefinable))
    }
}


fn to_list_items(vec : Vec<TmpObj>) -> CoreResult<Vec<ConstItem>>{
    let mut result : Vec<ConstItem> = Vec::with_capacity(vec.len());
    for item in vec{
        result.push(item.into_list_item()?);
    }
    return Ok(result);
}

fn to_data_items(vec : Vec<TmpObj>) -> CoreResult<HashM<String, ConstItem>>{
    let mut result : HashM<String, ConstItem> = HashMt::with_capacity(vec.len());
    for item in vec{
        let (s,m) = item.into_list_item_with_id()?;
        result.insert(s, m);
    }
    return Ok(result);
}

fn to_mut_list_items(vec : Vec<TmpObj>, next_id : u64) -> CoreResult<LinkedMap<MutItem>>{
    let mut result : Vec<(u64, MutItem)> = Vec::with_capacity(vec.len());
    for (idx, tmp_item) in vec.into_iter().enumerate(){
        //let span = tmp_item.span.clone();
        let pair = tmp_item.into_mut_list_item(idx)?;
        result.push(pair);
    }
    return Ok(LinkedMap::construct(result, next_id));
}