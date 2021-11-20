use std::marker::PhantomData;
use crate::imp::intf::CItemPtr;
use crate::imp::intf::clist::CListPtrIter;
use crate::imp::intf::clist_const::CItemConst;

unsafe impl<'a, V:From<CItemPtr>> Send for CListIterConst<'a, V>{}
unsafe impl<'a, V:From<CItemPtr>> Sync for CListIterConst<'a, V>{}

pub struct CListIterConst<'a, V : From<CItemPtr>>{
    ptr : CListPtrIter<V>,
    phantom : PhantomData<&'a i32>,
}

impl<'a, V : From<CItemPtr>> Iterator for CListIterConst<'a, V>{
    type Item = CItemConst<'a, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.next().map(|v|
            CItemConst::from_phantom(v, self.phantom))
    }
}

impl<'a, V : From<CItemPtr>> CListIterConst<'a, V>{
    pub fn new<T>(ptr : CListPtrIter<V>, _src : &'a T) -> CListIterConst<'a, V>{
        CListIterConst{ ptr, phantom : PhantomData }
    }
}