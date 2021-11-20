use crate::imp::structs::root_value::RootValue;
use crate::imp::structs::list_def_obj::{ ListDefMap};
use crate::imp::structs::list_value::ListDefValue;
use crate::imp::structs::ref_def_obj::RefDefMap;
use crate::imp::structs::param_type::ParamType;
use crate::imp::structs::var_type::VarType;
use crate::HashM;

unsafe impl Send for MetaTable{}
unsafe impl Sync for MetaTable{}

#[derive(Debug)]
pub struct MetaTable{
    table : Vec<Option<(*const String, MetaValue)>>
}

impl PartialEq for MetaTable{
    fn eq(&self, other: &Self) -> bool {
        if self.table.len() == other.table.len() {
            for i in 0..self.table.len() {
                let left = &self.table[i];
                let right = &other.table[i];
                if let Some(l) = left {
                    if let Some(r) = right {
                        unsafe{ if &*(l.0) != &*(r.0){ return false; } }
                        if l.1 != r.1 { return false; }
                    } else {
                        return false;
                    }
                } else {
                    if let Some(_) = right {
                        return false;
                    } else {}
                }
            }
            return true;
        } else {
            false
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct MetaTables{
    items : MetaTable,
    refs : MetaTable,
}

impl MetaTables{
    fn new(items : MetaTable, refs : MetaTable) -> MetaTables{ MetaTables{ items, refs }}
    pub fn items(&self) -> &MetaTable{ &self.items }
    pub fn refs(&self) -> &MetaTable{ &self.refs }
}

#[derive(Debug, PartialEq)]
pub enum MetaValue{
    Param(MetaParam),
    MList(MetaTables),
    OptMil(MetaTables)
}

#[derive(Debug, PartialEq)]
pub struct MetaParam{
    param_type : ParamType,
    var_type : VarType,
}
impl MetaParam{
    pub(crate) fn new(param_type : ParamType, var_type : VarType) -> MetaParam{
        MetaParam{ param_type, var_type }
    }
    pub fn param_type(&self) -> ParamType{ self.param_type }
    pub fn var_type(&self) -> VarType{ self.var_type }
}

impl MetaTable{
    pub(crate) fn new(table : Vec<Option<(*const String, MetaValue)>>) -> MetaTable{
        MetaTable{ table }
    }
    //pub(crate) fn table(&self) -> &Vec<(*const String, IsParam)>{ &self.table }

    pub fn get(&self, id : usize) -> Option<(&str, &MetaValue)>{
        if let Some(Some((key, p))) = self.table.get(id) {
            return Some((unsafe { (**key).as_str() }, p));
        }
        return None;
    }

    pub fn get_tables(&self, id : usize) -> Option<&MetaTables>{
        if let Some((_, val)) = self.get(id){
            return match val{
                MetaValue::MList(tables) => Some(tables),
                MetaValue::OptMil(tables) => Some(tables),
                MetaValue::Param(_) => None,
            };
        }
        return None;
    }
    ///Stringのポインタを得る都合上、ハッシュテーブルの再構築が行われれば、ポインタが無効となる。このハッシュテーブルは書き換えられてはいけない。
    pub(crate) fn from_root(root : &HashM<String, (usize, RootValue)>) -> MetaTable {
        let mut vec = init_vec(root.len());


        for (key, (id, val)) in root{
            match val{
                RootValue::Param(p,var) =>{
                    let val = MetaValue::Param(MetaParam::new(ParamType::from(p), *var));
                    vec[*id] = Some((key, val));
                },
                RootValue::MList(l) =>{
                    let def = l.default();
                    let val = if l.undefiable() {
                        MetaValue::OptMil(MetaTables::new(
                            Self::from_list_def(def.default()),
                            Self::from_ref_def(def.refs().refs())))
                    } else{
                        MetaValue::MList(MetaTables::new(
                            Self::from_list_def(def.default()),
                            Self::from_ref_def(def.refs().refs())))
                    };
                    vec[*id] = Some((key, val));
                },
                _ =>{},
            };
        }
        MetaTable::new(vec)
    }
    ///Stringのポインタを得る都合上、ハッシュテーブルの再構築が行われれば、ポインタが無効となる。このハッシュテーブルは書き換えられてはいけない。
    fn from_list_def(list_def : &ListDefMap) -> MetaTable{
        let mut vec : Vec<Option<(*const String, MetaValue)>> = init_vec(list_def.len());

        for (key,id, v) in list_def{
            match v{
                ListDefValue::Param(p,var) =>{
                    let val = MetaValue::Param(MetaParam::new(ParamType::from(p), *var));
                    vec[id] = Some((key, val));
                },
                ListDefValue::MilDef(mil) =>{
                    let def = mil.default();
                    if mil.undefiable(){
                        vec[id] = Some((key, MetaValue::OptMil(MetaTables::new(
                            Self::from_list_def(def.default()),
                            Self::from_ref_def(def.refs().refs())))));
                    } else{
                        vec[id] = Some((key, MetaValue::MList(MetaTables::new(
                            Self::from_list_def(def.default()),
                            Self::from_ref_def(def.refs().refs())))));
                    }
                },
                ListDefValue::CilDef(_)=>{},
            }
        }
        MetaTable::new(vec)
    }
    fn from_ref_def(ref_def : &RefDefMap) -> MetaTable{
        let mut vec : Vec<Option<(*const String, MetaValue)>> = init_vec(ref_def.len());

        for (key,id, v) in ref_def{
            vec[id] = Some((key, MetaValue::Param(MetaParam::new(ParamType::String, v.var_type()))))
        }
        MetaTable::new(vec)
    }
}

fn init_vec(len: usize) -> Vec<Option<(*const String, MetaValue)>>{
    let mut vec = Vec::with_capacity(len);
    for _ in 0..len{
        vec.push(None);
    }
    return vec;
}