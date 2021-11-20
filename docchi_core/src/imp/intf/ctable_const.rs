use std::marker::PhantomData;
use std::ops::Deref;

unsafe impl<'a, T> Send for CTableConst<'a, T>{}
unsafe impl<'a, T> Sync for CTableConst<'a, T>{}
#[derive(Debug)]
pub struct CTableConst<'a, T>{
    ptr : T,
    phantom : PhantomData<&'a i32>,
}

impl<'a, T> CTableConst<'a, T>{
    pub fn new<U>(ptr : T, _src : &'a U) -> CTableConst<'a, T>{
        CTableConst{ ptr, phantom : PhantomData }
    }
}

impl<'a, T> Deref for CTableConst<'a, T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}