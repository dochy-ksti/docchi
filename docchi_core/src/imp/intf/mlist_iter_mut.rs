use crate::imp::intf::MItemPtr;
use crate::imp::intf::mlist_ptr::MListPtrIter;
use std::marker::PhantomData;
use crate::intf::mlist_mut::MItemMut;

unsafe impl<'a, V:From<MItemPtr>> Send for MListIterMut<'a, V>{}
unsafe impl<'a, V:From<MItemPtr>> Sync for MListIterMut<'a, V>{}

#[derive(Debug)]
pub struct MListIterMut<'a, V : From<MItemPtr>>{
    ptr : MListPtrIter<V>,
    phantom : PhantomData<&'a mut i32>,
}

impl<'a, V : From<MItemPtr>> Iterator for MListIterMut<'a, V>{
    type Item = (u64, MItemMut<'a, V>);

    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.next_mut().map(|(id, v)| (id, MItemMut::from_phantom(v, self.phantom)))
    }
}
impl<'a, V : From<MItemPtr>> MListIterMut<'a, V>{
    pub fn new<T>(ptr : MListPtrIter<V>, _src : &'a mut T) -> MListIterMut<'a, V>{
        MListIterMut{ ptr, phantom : PhantomData }
    }
    pub fn next(&mut self) -> Option<(u64, MItemMut<'a, V>)> {
        self.ptr.next_mut().map(|(id, v)| (
            id,
            MItemMut::from_phantom(v, self.phantom)))
    }
    pub fn prev(&mut self) -> Option<(u64, MItemMut<'a, V>)> {
        self.ptr.next_mut().map(|(id, v)| (
            id,
            MItemMut::from_phantom(v, self.phantom)))
    }
    pub fn current(&mut self) -> Option<(u64, MItemMut<'a, V>)> {
        self.ptr.current_mut().map(|(id, v)| (
            id,
            MItemMut::from_phantom(v, self.phantom)))
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