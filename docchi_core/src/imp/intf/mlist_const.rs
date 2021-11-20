use crate::imp::intf::MItemPtr;
use crate::imp::intf::MListPtr;
use std::marker::PhantomData;
use std::ops::{Deref};
use crate::imp::intf::mlist_iter_const::MListIterConst;

unsafe impl<'a, V:From<MItemPtr>> Send for MListConst<'a, V>{}
unsafe impl<'a, V:From<MItemPtr>> Sync for MListConst<'a, V>{}
#[derive(Debug)]
pub struct MListConst<'a, V : From<MItemPtr>>{
    ptr : MListPtr<V>,
    phantom : PhantomData<&'a i32>,
}

impl<'a, V : From<MItemPtr>> MListConst<'a, V>{
    pub fn new<T>(ptr : MListPtr<V>, _src : &'a T) -> MListConst<'a, V>{
        MListConst{ ptr, phantom : PhantomData }
    }

    pub fn first(&self) -> Option<MItemConst<V>>{
        self.ptr.first_const().map(
            move |v| MItemConst::new(v, self))
    }
    pub fn first_id(&self) -> Option<u64>{
        self.ptr.first_id()
    }
    pub fn last(&self) -> Option<MItemConst<V>>{
        self.ptr.last_const().map(
            move |v| MItemConst::new(v, self))
    }
    pub fn last_id(&self) -> Option<u64>{
        self.ptr.last_id()
    }
    pub fn get_item(&self, id : u64) -> Option<MItemConst<V>>{
        self.ptr.get_item_const(id).map(
            move |v| MItemConst::new(v, self))
    }
    pub fn next_id(&self) -> u64{
        self.ptr.next_id()
    }
    pub fn contains_key(&self, key : u64) -> bool{
        self.ptr.contains_key(key)
    }
    pub fn len(&self) -> usize{
        self.ptr.len()
    }
    pub fn is_empty(&self) -> bool{
        self.ptr.is_empty()
    }

    pub fn iter(&self) -> MListIterConst<V>{
        MListIterConst::new(
        self.ptr.iter_const(), self)
    }
    pub fn iter_from_last(&self) -> MListIterConst<V>{
        MListIterConst::new(
            self.ptr.iter_from_last_const(), self)
    }
    pub fn iter_from_id(&self, id : u64) -> Option<MListIterConst<V>> {
        self.ptr.iter_from_id_const(id).map(
            |ptr| MListIterConst::new(ptr, self))
    }
}

unsafe impl<'a, V> Send for MItemConst<'a, V>{}
unsafe impl<'a, V> Sync for MItemConst<'a, V>{}
pub struct MItemConst<'a, V>{
    item : V,
    phantom : PhantomData<&'a i32>,
}

impl<'a, V> MItemConst<'a, V>{
    pub fn new<T>(item : V, _src : &'a T) -> MItemConst<'a, V>{
        MItemConst{ item, phantom : PhantomData }
    }
    pub fn from_phantom<T>(
        item : V,
        _src : PhantomData<&'a T>) -> MItemConst<'a, V>{
        MItemConst{ item, phantom : PhantomData }
    }
}

impl<'a, T> Deref for MItemConst<'a, T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}