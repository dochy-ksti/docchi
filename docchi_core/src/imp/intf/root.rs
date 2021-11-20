use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::qv::Qv;
use crate::imp::structs::root_value::RootValue;
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::rust_string::RustString;
use crate::imp::intf::clist::CListPtr;
use crate::imp::intf::table::TablePtr;
use crate::imp::intf::citem::CItemPtr;
use crate::imp::structs::rust_array::{RustIntArray, RustFloatArray};
use crate::structs::{RustBinary, MutListDef, MutListVal};
use crate::imp::structs::root_sab_value::RootSabValue;
use crate::imp::structs::root_def_obj::RootDefObj;
use crate::imp::intf::{MListPtr, MItemPtr};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RootObjectPtr{
    ptr : *mut RootObject
}
impl RootObjectPtr {
    pub fn new(ptr: *mut RootObject) -> RootObjectPtr { RootObjectPtr { ptr } }
    pub fn def(&self) -> *const RootDefObj{
        (unsafe{ &*self.ptr }).default()
    }
}

pub fn get_bool(root : RootObjectPtr, name : &str) -> Option<Qv<bool>>{
    if let Some(RustParam::Bool(b)) = get_param(root, name){
        Some(b.clone())
    } else{
        None
    }
}
pub fn get_bool_def(root : RootObjectPtr, name : &str) -> Option<Qv<bool>>{
    if let Some(RustParam::Bool(b)) = get_param_def(root, name){
        Some(b.clone())
    } else{
        None
    }
}

pub fn get_float(root : RootObjectPtr, name : &str) -> Option<Qv<f64>>{
    if let Some(RustParam::Float(b)) = get_param(root, name){
        Some(b.clone())
    } else{
        None
    }
}

pub fn get_float_def(root : RootObjectPtr, name : &str) -> Option<Qv<f64>>{
    if let Some(RustParam::Float(b)) = get_param_def(root, name){
        Some(b.clone())
    } else{
        None
    }
}

pub fn get_int(root : RootObjectPtr, name : &str) -> Option<Qv<i64>>{
    if let Some(RustParam::Int(b)) = get_param(root, name){
        Some(b.clone())
    } else{
        None
    }
}

pub fn get_int_def(root : RootObjectPtr, name : &str) -> Option<Qv<i64>>{
    if let Some(RustParam::Int(b)) = get_param_def(root, name){
        Some(b.clone())
    } else{
        None
    }
}

pub fn get_str(root : RootObjectPtr, name : &str) -> Option<Qv<String>>{
    if let Some(RustParam::String(b)) = get_param(root, name){
        Some(b.as_ref().map(|s| s.str().to_string()))
    } else{
        None
    }
}

pub fn get_str_def<'a, 'b>(root : RootObjectPtr, name : &'a str) -> Option<Qv<&'b String>>{
    if let Some(RustParam::String(b)) = get_param_def(root, name){
        match b{
            Qv::Val(v) => Some(Qv::Val(v.string())),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    } else{
        None
    }
}

pub fn get_int_array(root : RootObjectPtr, name : &str) -> Option<Qv<Vec<i64>>>{
    if let Some(RustParam::IntArray(b)) = get_param(root, name){
        Some(b.as_ref().map(|s| s.vec().clone()))
    } else{
        None
    }
}

pub fn get_int_array_def<'a, 'b>(root : RootObjectPtr, name : &'a str) -> Option<Qv<&'b Vec<i64>>>{
    if let Some(RustParam::IntArray(b)) = get_param_def(root, name){
        match b{
            Qv::Val(v) => Some(Qv::Val(v.vec())),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    } else{
        None
    }
}

pub fn get_float_array(root : RootObjectPtr, name : &str) -> Option<Qv<Vec<f64>>>{
    if let Some(RustParam::FloatArray(b)) = get_param(root, name){
        Some(b.as_ref().map(|s| s.vec().clone()))
    } else{
        None
    }
}

pub fn get_float_array_def<'a, 'b>(root : RootObjectPtr, name : &'a str) -> Option<Qv<&'b Vec<f64>>>{
    if let Some(RustParam::FloatArray(b)) = get_param_def(root, name){
        match b{
            Qv::Val(v) => Some(Qv::Val(v.vec())),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    } else{
        None
    }
}
pub fn get_binary(root : RootObjectPtr, name : &str) -> Option<Qv<Vec<u8>>>{
    if let Some(RustParam::Binary(b)) = get_param(root, name){
        Some(b.as_ref().map(|s| s.vec().clone()))
    } else{
        None
    }
}
pub fn get_binary_def<'a, 'b>(root : RootObjectPtr, name : &'a str) -> Option<Qv<&'b Vec<u8>>>{
    if let Some(RustParam::Binary(b)) = get_param_def(root, name){
        match b{
            Qv::Val(v) => Some(Qv::Val(v.vec())),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    } else{
        None
    }
}
pub fn get_immutable_binary<'a, 'b>(root : RootObjectPtr, name : &'a str) -> Option<Qv<&'b Vec<u8>>>{
    if let Some(RustParam::Binary(b)) = get_param(root, name){
        match b{
            Qv::Val(v) => Some(Qv::Val(v.vec())),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    } else{
        None
    }
}
pub fn get_immutable_str<'a, 'b>(root : RootObjectPtr, name : &'a str) -> Option<Qv<&'b String>>{
    if let Some(RustParam::String(b)) = get_param(root, name){
        match b{
            Qv::Val(v) => Some(Qv::Val(v.string())),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    } else{
        None
    }
}
pub fn get_immutable_int_array<'a, 'b>(root : RootObjectPtr, name : &'a str) -> Option<Qv<&'b Vec<i64>>>{
    if let Some(RustParam::IntArray(b)) = get_param(root, name){
        match b{
            Qv::Val(v) => Some(Qv::Val(v.vec())),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    } else{
        None
    }
}
pub fn get_immutable_float_array<'a, 'b>(root : RootObjectPtr, name : &'a str) -> Option<Qv<&'b Vec<f64>>>{
    if let Some(RustParam::FloatArray(b)) = get_param(root, name){
        match b{
            Qv::Val(v) => Some(Qv::Val(v.vec())),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    } else{
        None
    }
}

pub fn get_mutable_binary<'a, 'b>(ps : RootObjectPtr, name : &'a str) -> Option<Qv<&'b mut Vec<u8>>>{

    if let Some(RustParam::Binary(b)) = get_param_mut(ps, name){
        match b{
            Qv::Val(v) => Some(Qv::Val(v.vec_mut())),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    } else{
        None
    }
}
pub fn get_mutable_str<'a, 'b>(ps : RootObjectPtr, name : &'a str) -> Option<Qv<&'b mut String>>{

    if let Some(RustParam::String(b)) = get_param_mut(ps, name){
        match b{
            Qv::Val(v) => Some(Qv::Val(v.string_mut())),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    } else{
        None
    }
}
pub fn get_mutable_int_array<'a, 'b>(ps : RootObjectPtr, name : &'a str) -> Option<Qv<&'b mut Vec<i64>>>{
    if let Some(RustParam::IntArray(b)) = get_param_mut(ps, name){
        match b{
            Qv::Val(v) => Some(Qv::Val(v.vec_mut())),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    } else{
        None
    }
}
pub fn get_mutable_float_array<'a, 'b>(ps : RootObjectPtr, name : &'a str) -> Option<Qv<&'b mut Vec<f64>>>{
    if let Some(RustParam::FloatArray(b)) = get_param_mut(ps, name){
        match b{
            Qv::Val(v) => Some(Qv::Val(v.vec_mut())),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    } else{
        None
    }
}

pub fn get_table(root_def : *const RootDefObj, name : &str) -> Option<TablePtr>{
    let root = unsafe{ &*root_def };
    if let Some(RootValue::Table(d)) = root.get(name){
        Some(TablePtr::new(d, root_def))
    } else{ None }
}


pub fn get_clist<T : From<CItemPtr>>(root_ptr : RootObjectPtr, name : &str) -> Option<CListPtr<T>>{
    let root = unsafe{ &*root_ptr.ptr };
    if let Some(RootValue::CList(l)) = root.default().get(name){
        Some(CListPtr::new(l.list(), l.default(), root.default()))
    } else {
        None
    }
}

pub fn get_mlist_mut<T : From<MItemPtr>>(root : RootObjectPtr, name : &str) -> Option<MListPtr<T>>{
    let (def, sabun, _meta) = unsafe{  (*root.ptr).def_and_mut_sab() };
    if let Some(RootValue::MList(l)) = def.get(name){
        if let Some(v) = f::<T>(sabun.get_mut(name), l, def) {
            return Some(v);
        }
        sabun.insert(name.to_string(), RootSabValue::Mut(Some(MutListVal::crate_empty_list())));
        if let Some(v) = f::<T>(sabun.get_mut(name), l, def) {
            return Some(v);
        } else{
            unreachable!()
        }
    }
    return None;

    fn f<U : From<MItemPtr>>(sab : Option<&mut RootSabValue>, list_def : &MutListDef, root_def : &RootDefObj) -> Option<MListPtr<U>>{
        if let Some(RootSabValue::Mut(Some(m))) = sab {
            Some(MListPtr::new(m.list_mut(), list_def.default(), root_def))
        } else{
            None
        }
    }
}

pub fn get_mlist_const<T : From<MItemPtr>>(root : RootObjectPtr, name : &str) -> Option<Option<MListPtr<T>>>{
    let root = unsafe{ &*root.ptr };
    let (def, sabun) = (root.default(), root.sabun());
    if let Some(RootValue::MList(l)) = def.get(name){
        if let Some(RootSabValue::Mut(m)) = sabun.get(name) {
            if let Some(m) = m {
                return Some(Some(MListPtr::new(m.list(), l.default(), def)));
            } else{
                return Some(None);
            }
        }
    }
    return None;
}

pub fn get_param<'a>(ps : RootObjectPtr, name : &str) -> Option<&'a RustParam> {
    let ptr = unsafe{ &*ps.ptr };
    let def = ptr.default();
    let sab = ptr.sabun();

    if let Some(RootSabValue::Param(p)) = sab.get(name) {
        Some(p)
    } else if let Some(RootValue::Param(p, _v)) = def.get(name) {
        Some(p)
    } else { None }
}
pub fn get_param_mut<'a, 'b>(p : RootObjectPtr, name : &'a str) -> Option<&'b mut RustParam> {
    {
        let r = unsafe{ &mut *p.ptr };
        let (_def, m, _) = r.def_and_mut_sab();
        if let Some(RootSabValue::Param(p)) = m.get_mut(name) {
            return Some(p);
        }
    }

    let r = unsafe{ &mut *p.ptr };
    let (def, m, _) = r.def_and_mut_sab();
    if let Some(RootValue::Param(p,_)) = def.get(name){
        m.insert(name.to_string(), RootSabValue::Param(p.clone()));
    } else{
        return None;
    }
    if let Some(RootSabValue::Param(p)) = m.get_mut(name) {
        return Some(p);
    }
    unreachable!()
}
pub fn get_param_def<'a>(ps : RootObjectPtr, name : &str) -> Option<&'a RustParam> {
    let def = unsafe{ &*ps.ptr}.default();

    if let Some(RootValue::Param(p, _v)) = def.get(name) {
        Some(p)
    } else { None }
}

pub fn set_sabun(root : RootObjectPtr, name : &str, val : RustParam) -> bool{
    let root = unsafe{ &mut *root.ptr };
    match root.set_sabun_param(name.to_string(), val){
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn set_bool(root : RootObjectPtr, name : &str, val : Qv<bool>) -> bool{
    set_sabun(root, name, RustParam::Bool(val))
}
pub fn set_float(root : RootObjectPtr, name : &str, val : Qv<f64>) -> bool{
    set_sabun(root, name, RustParam::Float(val))
}
pub fn set_int(root : RootObjectPtr, name : &str, val : Qv<i64>) -> bool{
    set_sabun(root, name, RustParam::Int(val))
}
pub fn set_str(root : RootObjectPtr, name : &str, val : Qv<String>) -> bool{
    set_sabun(root,name, RustParam::String(val.map(|s| RustString::new(s))))
}
pub fn set_int_array(root : RootObjectPtr, name : &str, val : Qv<Vec<i64>>) -> bool{
    set_sabun(root,name, RustParam::IntArray(val.map(|s| RustIntArray::new(s))))
}
pub fn set_float_array(root : RootObjectPtr, name : &str, val : Qv<Vec<f64>>) -> bool{
    set_sabun(root,name, RustParam::FloatArray(val.map(|s| RustFloatArray::new(s))))
}
pub fn set_binary(root : RootObjectPtr, name : &str, val : Qv<Vec<u8>>) -> bool{
    set_sabun(root,name, RustParam::Binary(val.map(|s| RustBinary::new(s))))
}
/// Sets itinital value(0, empty string, zero-filled vec) to the parameter.
/// len is ignored except for vec-types.
/// This will be needed in the C interface.
pub fn set_initial_value<'a>(ps : RootObjectPtr, name : &str, len : usize) -> bool{
    let (def,sabun, _meta) =  unsafe{ &mut *ps.ptr }.def_and_mut_sab();

    if let Some(RootValue::Param(p, _)) = def.get(name) {
        sabun.insert(name.to_string(),RootSabValue::Param(p.to_default_value(len)));
        return true;
    } else{
        return false;
    }
}

/// Checks if the param hasn't been modified yet
pub fn is_unmodified(ps : RootObjectPtr, name : &str) -> bool{
    let item =  unsafe{ &mut *ps.ptr };

    !item.sabun().contains_key(name)
}