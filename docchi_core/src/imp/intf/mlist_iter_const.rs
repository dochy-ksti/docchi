use crate::imp::intf::MItemPtr;
use crate::imp::intf::mlist_ptr::MListPtrIter;
use std::marker::PhantomData;
use crate::imp::intf::mlist_const::MItemConst;

unsafe impl<'a, V:From<MItemPtr>> Send for MListIterConst<'a, V>{}
unsafe impl<'a, V:From<MItemPtr>> Sync for MListIterConst<'a, V>{}

pub struct MListIterConst<'a, V : From<MItemPtr>>{
    ptr : MListPtrIter<V>,
    phantom : PhantomData<&'a i32>,
}

impl<'a, V : From<MItemPtr>> Iterator for MListIterConst<'a, V>{
    type Item = (u64, MItemConst<'a, V>);

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

impl<'a, V : From<MItemPtr>> MListIterConst<'a, V>{
    pub fn new<T>(ptr : MListPtrIter<V>, _src : &'a T) -> MListIterConst<'a, V>{
        MListIterConst{ ptr, phantom : PhantomData }
    }

    pub fn next(&mut self) -> Option<(u64, MItemConst<'a, V>)> {
        self.ptr.next_const().map(|(id, v)| (
            id,
            MItemConst::from_phantom(v, self.phantom)))
    }
    pub fn prev(&mut self) -> Option<(u64, MItemConst<'a, V>)> {
        self.ptr.prev_const().map(|(id, v)| (
            id,
            MItemConst::from_phantom(v, self.phantom)))
    }
    pub fn current(&mut self) -> Option<(u64, MItemConst<'a, V>)> {
        self.ptr.current_const().map(|(id, v)| (
            id,
            MItemConst::from_phantom(v, self.phantom)))
    }
    pub fn is_available(&self) -> bool {
        self.ptr.is_available()
    }
    pub fn is_first(&self) -> bool {
        self.ptr.is_first()
    }
    pub fn is_last(&self) -> bool {
        self.ptr.is_last()
    }
}