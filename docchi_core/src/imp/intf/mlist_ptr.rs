use crate::imp::structs::linked_m::{LinkedMap};
use crate::imp::structs::rust_list::MutItem;
use std::marker::PhantomData;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::intf::mitem::MItemPtr;
use crate::imp::structs::root_def_obj::RootDefObj;
use crate::imp::structs::linked_map_unsafe_iter::LinkedMapUnsafeIter;

/// This uses pointers so every method is basically unsafe.
/// You can get this ptr, and create an immutable reference,
/// and modify the referent through this pointer,
/// and access the immutable reference afterwards.
/// Anything can happen with the access.
///
/// Getting data through this pointer while a mutable reference is alive
/// is also an undefined behavior.
///
/// Pointers can outlive their referents, and access dropped items. It's also UB.
///
/// Because references and pointers are not exposed,
/// creating contradict references is basically impossible.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MListPtr<V : From<MItemPtr>>{
    map : *const LinkedMap<MutItem>,
    list_def : *const ListDefObj,
    root_def : *const RootDefObj,
    phantom : PhantomData<*const V>,
}

impl<V : From<MItemPtr>> MListPtr<V>{
    /// &LinkedMap<MutItem>から得たポインタを通して書き換えるとUBなので、
    /// 書き換える場合&mut LinkedMapからポインタを取得し、*constに入れる
    /// Const版とMut版に分けて安全に書くのも出来なくはない気がするが大変すぎるのでこのようになっている
    pub fn new(map : *const LinkedMap<MutItem>, list_def : *const ListDefObj, root_def : *const RootDefObj)
        -> MListPtr<V>{ MListPtr { map, list_def, root_def, phantom : PhantomData } }

    fn from(&self, item : *const MutItem) -> V{
        V::from(MItemPtr::new(item, self.list_def, self.root_def))
    }

    fn map(&self) -> &LinkedMap<MutItem>{ unsafe{ &*self.map }}
    fn map_mut<'a>(&mut self) -> &'a mut LinkedMap<MutItem>{ unsafe{ &mut *(self.map as *mut LinkedMap<MutItem>) }}
    //fn def_ref<'a>(&mut self) -> &'a ListDefObj{ unsafe{ &*self.list_def } }

    pub fn first_mut(&mut self) -> Option<V> {
        let map = self.map_mut();
        map.first_mut().map(|r| self.from(r))
    }

    pub fn first_const(&self) -> Option<V> {
        let map = self.map();
        map.first().map(|r| self.from(r))
    }
    pub fn first_id(&self) -> Option<u64> {
        self.map().first_id()
    }
    pub fn last_mut(&mut self) -> Option<V> {
        self.map_mut().last_mut().map(|r| self.from(r))
    }

    pub fn last_const(&self) -> Option<V> {
        self.map().last().map(|r| self.from(r))
    }

    pub fn last_id(&self) -> Option<u64> {
        self.map().last_id()
    }
    pub fn get_item_mut(&mut self, id : u64) -> Option<V>{
        self.map_mut().get_item_mut(id).map(|b| self.from(b))
    }

    pub fn get_item_const(&self, id : u64) -> Option<V>{
        self.map().get_item(id).map(|b| self.from(b))
    }

    pub fn next_id(&self) -> u64{
        self.map().next_id()
    }

    pub fn contains_key(&self, key : u64) -> bool{
        self.map().contains_key(key)
    }
    pub fn len(&self) -> usize{
        self.map().len()
    }
    pub fn is_empty(&self) -> bool {
        self.map().is_empty()
    }

    pub fn insert(&mut self) -> V{
        self.insert_last()
    }

    pub fn insert_last(&mut self) -> V{
        let map = self.map_mut();
        let id = map.insert_last(MutItem::default());
        self.get_item_mut(id).unwrap()
    }
    pub fn insert_first(&mut self) -> V{
        let map = self.map_mut();
        let id = map.insert_first(MutItem::default());
        self.get_item_mut(id).unwrap()
    }

    /// Anything can happen when a removed item is accessed, so be careful
    pub unsafe fn remove(&mut self, id : u64) -> bool {
        self.map_mut().remove(id)
    }
    /// Anything can happen when a removed item is accessed, so be careful
    pub unsafe fn remove_first(&mut self) -> bool{
        self.map_mut().remove_first()
    }
    /// Anything can happen when a removed item is accessed, so be careful
    pub unsafe fn remove_last(&mut self) -> bool{
        self.map_mut().remove_last()
    }

    pub fn move_to_first(&mut self, id : u64) -> bool {
        self.map_mut().move_to_first(id)
    }

    pub fn move_to_last(&mut self, id : u64) -> bool {
        self.map_mut().move_to_last(id)
    }

    pub fn move_to_prev(&mut self, next_items_id : u64, id : u64) -> bool{
        self.map_mut().move_to_prev(next_items_id, id)
    }

    pub fn move_to_next(&mut self, prev_items_id : u64, id : u64) -> bool{
        self.map_mut().move_to_next(prev_items_id, id)
    }

    pub fn iter_const(&self) -> MListPtrIter<V> {
        let iter  = unsafe{ self.map().iter_unsafe_const() };
        MListPtrIter::new(iter, self.list_def, self.root_def)
    }
    pub fn iter_mut(&mut self) -> MListPtrIter<V> {
        let iter = unsafe{ self.map_mut().iter_unsafe_mut() };
        MListPtrIter::new(iter, self.list_def, self.root_def)
    }

    pub fn iter_from_last_const(&self) -> MListPtrIter<V> {
        let iter = unsafe{ self.map().iter_from_last_unsafe_const() };
        MListPtrIter::new(iter, self.list_def, self.root_def)
    }
    pub fn iter_from_last_mut(&mut self) -> MListPtrIter<V>{
        let iter = unsafe{ self.map_mut().iter_from_last_unsafe_mut() };
        MListPtrIter::new(iter, self.list_def, self.root_def)
    }

    pub fn iter_from_id_const(&self, id : u64) -> Option<MListPtrIter<V>> {
        let iter = unsafe{ self.map().iter_from_id_unsafe_const(id) };
        iter.map(|iter| MListPtrIter::new(iter, self.list_def, self.root_def))
    }
    pub fn iter_from_id_mut(&mut self, id : u64)-> Option<MListPtrIter<V>> {
        let iter = unsafe{ self.map_mut().iter_from_id_unsafe_mut(id) };
        iter.map(|iter| MListPtrIter::new(iter, self.list_def, self.root_def))
    }
}

#[derive(Debug)]
pub struct MListPtrIter<V : From<MItemPtr>>{
    iter : LinkedMapUnsafeIter<MutItem>,
    list_def : *const ListDefObj,
    root_def : *const RootDefObj,
    phantom : PhantomData<*mut V>,
}
impl<V : From<MItemPtr>> Iterator for MListPtrIter<V>{
    type Item = (u64, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next_mut().map(|(k,v)| (*k, V::from(MItemPtr::new(v, self.list_def, self.root_def))))
    }
}
impl<V : From<MItemPtr>> MListPtrIter<V>{
    pub(crate) fn new(iter : LinkedMapUnsafeIter<MutItem>, list_def : *const ListDefObj, root_def : *const RootDefObj) -> MListPtrIter<V>{
        MListPtrIter { iter, list_def, root_def, phantom : PhantomData }
    }

    fn from(&self, item : *const MutItem) -> V{
        V::from(MItemPtr::new(item, self.list_def, self.root_def))
    }
    ///現在のカーソルにあるアイテムを返し、カーソルを進める
    pub fn next_const(&mut self) -> Option<(u64, V)> {
        self.iter.next_const().map(|(k,v)| (*k, self.from(v)))
    }

    ///現在のカーソルにあるアイテムを返し、カーソルを進める。Mutate可能なMItemPtrを返すをラップして返す。
    /// ConstなMListPtrから得たMListPtrIterから呼び出して書き換えたらUB
    pub fn next_mut(&mut self) -> Option<(u64, V)> {
        self.iter.next_mut().map(|(k,v)| (*k, self.from(v)))
    }

    ///前に戻ることが出来る。そして元あった場所を削除し、それによって削除されたアイテムの次にあったアイテムが現在のカーソルの次にくるので、
    /// next2回でそれをとることも出来る。
    ///今ある場所をremoveしたらポインタが不正になって安全にnext/prevできない
    pub fn prev_const(&mut self) -> Option<(u64, V)> {
        self.iter.prev_const().map(|(k,v)| (*k, self.from(v)))
    }

    pub fn prev_mut(&mut self) -> Option<(u64, V)> {
        self.iter.prev_mut().map(|(k,v)| (*k, self.from(v)))
    }
    
    pub fn current_const(&mut self) -> Option<(u64, V)> {
        self.iter.current_const().map(|(k,v)| (*k,self.from(v)))
    }

    pub fn current_mut(&mut self) -> Option<(u64, V)> {
        self.iter.current_mut().map(|(k,v)| (*k,self.from(v)))
    }



    pub fn is_available(&self) -> bool {
        self.iter.is_available()
    }

    pub fn is_first(&self) -> bool {
        self.iter.is_first()
    }

    pub fn is_last(&self) -> bool {
        self.iter.is_last()
    }
}
