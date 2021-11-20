use crate::{HashM, IdentityEqual};
use crate::imp::structs::root_value::RootValue;
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::util::set_sabun::{SetSabunError, verify_set_sabun};
use crate::imp::structs::meta_table::MetaTable;
use crate::imp::structs::util::hash_m::HashS;
use std::sync::{Arc, Weak};
use crate::imp::structs::root_sab_value::RootSabValue;
use crate::imp::structs::root_def_obj::RootDefObj;

#[derive(Debug)]
pub struct RootObject{
    ///listのobjectの場合、defaultはlist側にあるが、ここには初期値が入る。
    default : Arc<RootDefObj>,
    ///変更されたものを記録
    ///listの変更はMutListが直接上書きされるので、sabunには入らない。よってparamだけ記録される
    sabun : Arc<HashM<String, RootSabValue>>,

    ///oldに設定されたメンバは、_Oldを付けなければプログラムから使用できず、
    ///ConstTableである場合、jsonで Refできない
    old : Arc<HashS<String>>,

    id : Arc<()>,

    meta_table : Arc<MetaTable>,

}

// impl PartialEq for RootObject{
//     fn eq(&self, other: &Self) -> bool {
//         //meta tableは既存の情報を整理しただけで情報量的に変わりがないから等価性には関わらない
//         self.default == other.default && self.sabun == other.sabun && self.old == other.old
//     }
// }

impl Clone for RootObject{
    fn clone(&self) -> Self {
        let default = self.default.clone();
        let sabun = self.sabun.clone();
        let old  = self.old.clone();
        let meta_table = self.meta_table.clone();

        //idはclone時も新調する
        let id = Arc::new(());
        Self{ default, sabun, old, meta_table, id }
    }
}

impl RootObject{
    pub fn new(default : RootDefObj, sabun : HashM<String, RootSabValue>, old : HashS<String>) -> RootObject{
        let meta_table = MetaTable::from_root(default.def());
        RootObject{ default: Arc::new(default), sabun : Arc::new(sabun), old : Arc::new(old), meta_table : Arc::new(meta_table), id : Arc::new(()) }
    }
    pub fn default(&self) -> &RootDefObj{ self.default.as_ref() }
    pub fn default_arc(&self) -> Arc<RootDefObj>{ self.default.clone() }

    pub fn meta_table(&self) -> &MetaTable{ self.meta_table.as_ref() }
    pub fn meta_table_arc(&self) -> Arc<MetaTable>{ self.meta_table.clone() }

    /////mlistがdefaultにある都合上、書き換える必要性が生じている。HashMのKeyはmetatableからポインタ参照されているので、ハッシュ再構成が起きてはならない
    //pub(crate) fn default_mut(&mut self) -> &mut HashM<String, (usize, RootValue)>{ self.default.as_mut() }

    pub fn def_and_mut_sab(&mut self) -> (&RootDefObj,
                                          &mut HashM<String, RootSabValue>,
                                          &MetaTable){
         (&self.default, Arc::make_mut(&mut self.sabun), &self.meta_table)
    }

    pub fn deconstruct(self)
        -> (Arc<RootDefObj>, Arc<HashM<String, RootSabValue>>,
            Arc<HashS<String>>, Arc<MetaTable>){
        (self.default, self.sabun, self.old, self.meta_table)
    }
    pub fn construct(default : Arc<RootDefObj>,
                     sabun : Arc<HashM<String, RootSabValue>>,
                     old : Arc<HashS<String>>,
                     meta_table : Arc<MetaTable>) -> RootObject{
        RootObject{ default, sabun, old, meta_table, id : Arc::new(()) }
    }
    pub fn sabun(&self) -> &HashM<String, RootSabValue>{ self.sabun.as_ref() }
    pub fn sabun_mut(&mut self) -> &mut HashM<String, RootSabValue>{ Arc::make_mut(&mut self.sabun) }
    pub fn old(&self) -> &HashS<String>{ self.old.as_ref() }
    pub fn old_arc(&self) -> Arc<HashS<String>>{ self.old.clone() }

    pub fn set_sabun_param(&mut self, name : String, param : RustParam) -> Result<Option<RustParam>, SetSabunError> {
        let (p, vt) = if let Some(RootValue::Param(p, vt)) = self.default().get(&name) { (p, vt) } else {
            return Err(SetSabunError::ParamNotFound);
        };
        verify_set_sabun(p, vt, &param)?;
        let sabun = Arc::make_mut(&mut self.sabun);
        if let Some(sab) = sabun.insert(name, RootSabValue::Param(param)){
            if let RootSabValue::Param(p) = sab {
                Ok(Some(p))
            } else{
                Err(SetSabunError::ParamNotFound)
            }
        } else{
            Ok(None)
        }
    }

    // /// checks any sabun has been modified
    // pub fn contents_eq(&self, other : &Self) -> bool{
    //     Arc::ptr_eq(self.sabun(), other.sabun())
    // }

    pub fn id(&self) -> Weak<()>{
        Arc::downgrade(&self.id)
    }

    pub fn id_ptr_eq(&self, id : &Weak<()>) -> bool{
        //WeakもArcも生きている。勝手にdropしたりしない
        id.as_ptr() == Arc::as_ptr(&self.id)
    }

    pub fn contents_ptr_eq(&self, other : &Self) -> bool{
        Arc::ptr_eq(&self.sabun, &other.sabun)
    }

    pub fn contents_eq(&self, other : &Self) -> bool{
        self.sabun.identity_eq(&other.sabun)
    }
}


