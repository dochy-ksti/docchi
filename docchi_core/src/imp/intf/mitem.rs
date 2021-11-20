use crate::imp::structs::rust_list::MutItem;
use crate::imp::structs::list_value::{ListDefValue};
use crate::imp::structs::qv::{Qv};
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::intf::{CItemPtr, MListPtr};
use crate::imp::structs::ref_value::RefSabValue;
use crate::imp::intf::citem::{get_enum_impl, get_ref_id_impl};
use crate::imp::structs::rust_string::RustString;
use crate::imp::structs::rust_array::{RustIntArray, RustFloatArray};
use crate::structs::{RustBinary, MutListDef};
use crate::imp::structs::list_sab_value::ListSabValue;
use crate::imp::structs::root_def_obj::RootDefObj;
use crate::imp::structs::rust_list::{MutListVal};

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MItemPtr {
    item : *const MutItem,
    list_def : *const ListDefObj,
    root_def: *const RootDefObj,
}

impl MItemPtr {
    ///&mut MutItemから得たポインタであれば、*constであっても*mutにキャストして&mut参照を得て書き換えても良い
    /// &から得たポインタを通じて書き換えるとUB。&mut参照を構築することすらもUBではないかという説もあるが、それは違うと思う
    /// &mut参照と&参照が同時に存在することもかなりUB的であるが、それも厳密には違うように思う。
    /// ただし、このシステムは参照を露出しないので、シングルスレッドアプリにおいて同一メモリアドレスに２つ以上の参照が構築されることはないはず
    /// マルチスレッド化はArc::make_mutを用いて行われるので、マルチスレッドで同一アドレスへのアクセスが有る場合、
    /// &mutを得る時はmake_mutによりコピーが行われるので、&mutと&参照がかぶることもないはず
    /// このPtrが正しくラップされ、ライフタイムルールが壊れていなければ、シングルスレッドにおいて&mut と &が同時に存在することは出来ないが、
    /// そもそもそれ以前に参照を露出しなければ、シングルスレッドでは参照を複数存在させることは基本的に無理である
    pub fn new(item : *const MutItem, list_def : *const ListDefObj, root_def : *const RootDefObj) -> MItemPtr {
        MItemPtr { item, list_def, root_def }
    }
    pub fn item(&self) -> *const MutItem { self.item }
    /// *const MutItem must be obtained from &mut when this is mutated
    unsafe fn item_mut<'a>(&self) -> &'a mut MutItem { &mut *(self.item as *mut _) }
    pub fn list_def(&self) -> *const ListDefObj{ self.list_def }

}

pub fn get_mil_mut<T : From<MItemPtr>>(ps : MItemPtr, name : &str) -> Option<MListPtr<T>> {
    let (item, list_def) = unsafe { (ps.item_mut(), &*ps.list_def) };
    if let Some(ListDefValue::MilDef(md)) = list_def.default().get(name) {
        if let Some(v) = f::<T>(item.values_mut().get_mut(name), md, ps){
            return Some(v);
        }

        item.values_mut().insert(name.to_string(), ListSabValue::Mil(Some(MutListVal::crate_empty_list())));
        if let Some(v) = f::<T>(item.values_mut().get_mut(name), md, ps){
            return Some(v);
        } else{
            unreachable!()
        }
    } else{
        return None;
    }

    fn f<U : From<MItemPtr>>(sab_value : Option<&mut ListSabValue>, md : &MutListDef, ps : MItemPtr) -> Option<MListPtr<U>>{
        if let Some(ListSabValue::Mil(Some(l))) =  sab_value {
            Some(MListPtr::new(l.list_mut(), md.default(), ps.root_def))
        } else{
            None
        }
    }
}

/// Mutation through this ptr is undefined behavior
pub fn get_mil_const<T : From<MItemPtr>>(ps : MItemPtr, name : &str) -> Option<Option<MListPtr<T>>> {
    let (item, list_def) = unsafe { (&*ps.item, &*ps.list_def) };
    if let Some(ListDefValue::MilDef(md)) = list_def.default().get(name) {
        if let Some(ListSabValue::Mil(data)) = item.values().get(name) {
            if let Some(inner) = data {
                return Some(Some(MListPtr::new(inner.list(), md.default(), ps.root_def)))
            } else {
                return Some(None);
            }
        } else{
            return Some(Some(MListPtr::new(
                MutListVal::empty_lists_ref(),
                md.default(), ps.root_def)));
        }
    }
    return None
}


pub fn get_bool(ps : MItemPtr, name : &str) -> Option<Qv<bool>>{
    if let Some(RustParam::Bool(b)) = get_param(ps, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_bool_def(ps : MItemPtr, name : &str) -> Option<Qv<bool>>{
    if let Some(RustParam::Bool(b)) = get_param_def(ps, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_int(ps : MItemPtr, name : &str) -> Option<Qv<i64>>{
    if let Some(RustParam::Int(b)) = get_param(ps, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_int_def(ps : MItemPtr, name : &str) -> Option<Qv<i64>>{
    if let Some(RustParam::Int(b)) = get_param_def(ps, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_float(ps : MItemPtr, name : &str) -> Option<Qv<f64>>{
    if let Some(RustParam::Float(b)) = get_param(ps, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_float_def(ps : MItemPtr, name : &str) -> Option<Qv<f64>>{
    if let Some(RustParam::Float(b)) = get_param_def(ps, name){
        Some(b.clone())
    } else{ None }
}

pub fn get_str(ps : MItemPtr, name : &str) -> Option<Qv<String>>{
    if let Some(RustParam::String(b)) = get_param(ps, name){
        Some(b.as_ref().map(|s| s.str().to_string()))
    } else{ None }
}

pub fn get_str_def<'a, 'b>(ps : MItemPtr, name : &'a str) -> Option<Qv<&'b String>>{
    if let Some(RustParam::String(b)) = get_param_def(ps, name){
        match b{
            Qv::Val(v) => Some(Qv::Val(v.string())),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    } else{ None }
}

pub fn get_int_array(ps : MItemPtr, name : &str) -> Option<Qv<Vec<i64>>>{
    if let Some(RustParam::IntArray(b)) = get_param(ps, name){
        Some(b.as_ref().map(|s| s.vec().clone()))
    } else{
        None
    }
}

pub fn get_int_array_def<'a, 'b>(ps : MItemPtr, name : &'a str) -> Option<Qv<&'b Vec<i64>>>{
    if let Some(RustParam::IntArray(b)) = get_param_def(ps, name){
        match b{
            Qv::Val(v) => Some(Qv::Val(v.vec())),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    } else{
        None
    }
}
pub fn get_float_array(ps : MItemPtr, name : &str) -> Option<Qv<Vec<f64>>>{
    if let Some(RustParam::FloatArray(b)) = get_param(ps, name){
        Some(b.as_ref().map(|s| s.vec().clone()))
    } else{
        None
    }
}
pub fn get_float_array_def<'a, 'b>(ps : MItemPtr, name : &'a str) -> Option<Qv<&'b Vec<f64>>>{
    if let Some(RustParam::FloatArray(b)) = get_param_def(ps, name){
        match b{
            Qv::Val(v) => Some(Qv::Val(v.vec())),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    } else{
        None
    }
}
pub fn get_binary(ps : MItemPtr, name : &str) -> Option<Qv<Vec<u8>>>{
    if let Some(RustParam::Binary(b)) = get_param(ps, name){
        Some(b.as_ref().map(|s| s.vec().clone()))
    } else{
        None
    }
}
pub fn get_binary_def<'a, 'b>(ps : MItemPtr, name : &'a str) -> Option<Qv<&'b Vec<u8>>>{
    if let Some(RustParam::Binary(b)) = get_param_def(ps, name){
        match b{
            Qv::Val(v) => Some(Qv::Val(v.vec())),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    } else{
        None
    }
}

pub fn get_immutable_binary<'a, 'b>(ps : MItemPtr, name : &'a str) -> Option<Qv<&'b Vec<u8>>>{
    if let Some(RustParam::Binary(b)) = get_param(ps, name){

        match b{
             Qv::Val(v) => Some(Qv::Val(v.vec())),
             Qv::Null => Some(Qv::Null),
             Qv::Undefined => Some(Qv::Undefined)
        }
    } else{
        None
    }
}

pub fn get_immutable_str<'a, 'b>(ps : MItemPtr, name : &'a str) -> Option<Qv<&'b String>>{
    if let Some(RustParam::String(b)) = get_param(ps, name){
        match b{
            Qv::Val(v) => Some(Qv::Val(v.string())),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    } else{
        None
    }
}
pub fn get_immutable_int_array<'a, 'b>(ps : MItemPtr, name : &'a str) -> Option<Qv<&'b Vec<i64>>>{
    if let Some(RustParam::IntArray(b)) = get_param(ps, name){
        match b{
            Qv::Val(v) => Some(Qv::Val(v.vec())),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    } else{
        None
    }
}
pub fn get_immutable_float_array<'a, 'b>(ps : MItemPtr, name : &'a str) -> Option<Qv<&'b Vec<f64>>>{
    if let Some(RustParam::FloatArray(b)) = get_param(ps, name){
        match b{
            Qv::Val(v) => Some(Qv::Val(v.vec())),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    } else{
        None
    }
}

/// Vec のメモリは 最低でも8byte境界にalignされているはず(Rustはそれを保証していないが、それをしないallocatorは存在してないはず)
/// なので、slice::align_to_mut で u64 や i32 のsliceを取得してゴリゴリ書き換えることも可能なはず
/// vec.len が sizeof の倍数ならば、align_toしてもprefixやsuffixは発生しないだろう。(align_to_mutはそれを保証していないが)
///
/// 常にCとの相互運用性を考えているのだが、結局ポインタを渡してしまえばCのノリで何も考えずにキャストされゴリゴリ書き換えられるだろうから、
/// align_toのこととか考えても無駄だろう
///
/// little endianのマシンから big endianのマシンに移したデータで問題が起きたりとかそういうことはあるだろうが、
/// 現段階ではそんなこと知ったことではないと思う。十分に無視するに値するコーナーケースと考える。
///
/// Boxから取られた参照なので、理論的には、Boxの中のVecが置き換えられても、この参照自体は無効にはならないはずだ。
/// しかしVecが参照しているSliceは無効になる。Sliceの参照が生きている間に置き換えが発生すればUBだ。
///
/// だからVecからSliceを取り出すときには注意したほうが良い・・・。Cインターフェースを作る時に特に注意すべきである
///
/// まあRustはBoxの中の値が置き換えられたときBox内への参照が有効であることは保証していないと思うが・・・
pub fn get_mutable_binary<'a, 'b>(ps : MItemPtr, name : &'a str) -> Option<Qv<&'b mut Vec<u8>>>{
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
pub fn get_mutable_str<'a, 'b>(ps : MItemPtr, name : &'a str) -> Option<Qv<&'b mut String>>{
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
pub fn get_mutable_int_array<'a, 'b>(ps : MItemPtr, name : &'a str) -> Option<Qv<&'b mut Vec<i64>>>{
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
pub fn get_mutable_float_array<'a, 'b>(ps : MItemPtr, name : &'a str) -> Option<Qv<&'b mut Vec<f64>>>{
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

pub fn set_bool(ps : MItemPtr, name : &str, val : Qv<bool>) -> bool{
    set_sabun(ps, name, RustParam::Bool(val))
}
pub fn set_int(ps : MItemPtr, name : &str, val : Qv<i64>) -> bool{
    set_sabun(ps, name, RustParam::Int(val))
}
pub fn set_float(ps : MItemPtr, name : &str, val : Qv<f64>) -> bool{
    set_sabun(ps, name, RustParam::Float(val))
}
pub fn set_str(ps : MItemPtr, name : &str, val : Qv<String>) -> bool{
    set_sabun(ps, name, RustParam::String(val.map(|s| RustString::new(s))))
}
pub fn set_int_array(ps : MItemPtr, name : &str, val : Qv<Vec<i64>>) -> bool{
    set_sabun(ps, name, RustParam::IntArray(val.map(|s| RustIntArray::new(s))))
}
pub fn set_float_array(ps : MItemPtr, name : &str, val : Qv<Vec<f64>>) -> bool{
    set_sabun(ps, name, RustParam::FloatArray(val.map(|s| RustFloatArray::new(s))))
}
pub fn set_binary(ps : MItemPtr, name : &str, val : Qv<Vec<u8>>) -> bool{
    set_sabun(ps, name, RustParam::Binary(val.map(|s| RustBinary::new(s))))
}

pub fn set_sabun(ps : MItemPtr, name : &str, p : RustParam) -> bool {
    let (item, def) = unsafe { (ps.item_mut(), &*ps.list_def) };
    match item.set_sabun(def, name.to_string(), p) {
        Ok(_) => true,
        Err(_) => false,
    }
}


pub fn get_param<'a>(ps : MItemPtr, name : &str) -> Option<&'a RustParam>{
    let (item, def) = unsafe{ (&*ps.item, &*ps.list_def) };
    if let Some(ListSabValue::Param(p)) = item.values().get(name){
        Some(p)
    } else if let Some(ListDefValue::Param(p, _)) = def.default().get(name){
        Some(p)
    } else{
        None
    }
}

pub fn get_param_def<'a>(ps : MItemPtr, name : &str) -> Option<&'a RustParam>{
    let def = unsafe{ &*ps.list_def };
    if let Some(ListDefValue::Param(p, _)) = def.default().get(name){
        Some(p)
    } else{
        None
    }
}

/// 差分がない場合、デフォルト値をコピーして差分にツッコミ、さらにその&mut を返す
pub fn get_param_mut<'a>(ps : MItemPtr, name : &str) -> Option<&'a mut RustParam> {
    let (def, item) = unsafe { (&*ps.list_def, ps.item_mut()) };
    if let Some(ListSabValue::Param(p)) = item.values_mut().get_mut(name) {
        return Some(p);
    }
    let item = unsafe { ps.item_mut() }; //なんでこうしないとコンパイラに怒られるのかはよくわからない
    if let Some(ListDefValue::Param(p, _)) = def.default().get(name) {
        item.values_mut().insert(name.to_string(), ListSabValue::Param(p.clone()));
    } else {
        return None;
    }
    if let Some(ListSabValue::Param(p)) = item.values_mut().get_mut(name) {
        Some(p)
    } else {
        unreachable!()
    }
}

pub fn set_ref(ps : MItemPtr, list_name : &str, id : Qv<String>) -> bool{
    let item = unsafe{ ps.item_mut() };
    item.refs_mut().insert(list_name.to_string(), RefSabValue::new(id));
    return true;
}

pub fn get_ref(ps : MItemPtr, list_name : &str) -> Option<Qv<CItemPtr>>{
    let qv = get_ref_id(ps, list_name)?;
    qv.opt_map(|id|{
        let data = super::root::get_table(ps.root_def, list_name).unwrap();
        super::table::get_value(data, id)
    })
}

pub fn get_ref_id<'a>(ps : MItemPtr, list_name : &str) -> Option<Qv<&'a String>>{
    let (item, list_def) = unsafe{ (&*ps.item, &*ps.list_def) };
    get_ref_id_impl(item.refs(), list_def, list_name)
}

pub fn get_enum(ps : MItemPtr) -> Option<(String, String)>{
    let item = unsafe{ &*ps.item };
    get_enum_impl(item.refs())
}

pub fn set_enum(ps : MItemPtr, list_name : &str, id : &str) -> bool{
    let item = unsafe{ ps.item_mut() };
    item.refs_mut().clear();
    set_ref(ps, list_name, Qv::Val(id.to_string()))
}

/// Sets itinital value(0, empty string, zero-filled vec) to the parameter.
/// len is ignored except for vec-types.
/// This should be needed in the C interface.
pub fn set_initial_value<'a>(ps : MItemPtr, name : &str, len : usize) -> bool{
    let (def, item) = unsafe { (&*ps.list_def, ps.item_mut()) };
    if let Some(ListDefValue::Param(p, _)) = def.default().get(name) {
        item.values_mut().insert(name.to_string(), ListSabValue::Param(p.to_default_value(len)));
        return true;
    } else{
        return false;
    }
}

/// Checks if the param hasn't been modified yet
pub fn is_unmodified(ps : MItemPtr, name : &str) -> bool{
    let item = unsafe { &*ps.item };
    !item.values().contains_key(name)
}