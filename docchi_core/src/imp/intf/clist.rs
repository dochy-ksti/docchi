use crate::imp::intf::citem::CItemPtr;
use crate::imp::structs::list_def_obj::ListDefObj;
use std::marker::PhantomData;
use crate::imp::structs::rust_list::ConstItem;
use crate::imp::structs::root_def_obj::RootDefObj;

/// CList's internal structure is Vec
#[derive(Debug, PartialEq)]
pub struct CListPtr<T : From<CItemPtr>> {
    ptr : *const Vec<ConstItem>,
    list_def : *const ListDefObj,
    root : *const RootDefObj,
    phantom : PhantomData<*mut T>,
}
impl<T : From<CItemPtr>> Clone for CListPtr<T>{
    fn clone(&self) -> Self {
        CListPtr::new(self.ptr, self.list_def, self.root)
    }
}
impl<T : From<CItemPtr>> Copy for CListPtr<T>{}

impl<T : From<CItemPtr>> CListPtr<T> {
    pub fn new(ptr : *const Vec<ConstItem>, list_def : *const ListDefObj, root : *const RootDefObj) -> CListPtr<T> { CListPtr { ptr, list_def, root, phantom : PhantomData } }
    pub fn len(&self) -> usize{ get_len(self.clone()) }
    pub fn value(&self, idx : usize) -> T{ get_value(self.clone(), idx) }
    pub fn iter(&self) -> CListPtrIter<T>{ get_iter(self.clone()) }
}

pub fn get_len<T : From<CItemPtr>>(list: CListPtr<T>) -> usize{
    let v = unsafe{ list.ptr.as_ref().unwrap()};
    v.len()
}

pub fn get_value<T : From<CItemPtr>>(list: CListPtr<T>, idx : usize) -> T{
    let vec = unsafe{ list.ptr.as_ref().unwrap()};
    T::from(CItemPtr::new(&vec[idx], list.list_def, list.root))
}

pub fn get_iter<T : From<CItemPtr>>(list: CListPtr<T>) -> CListPtrIter<T>{
    CListPtrIter::new(list.ptr, list.list_def, list.root)
}

pub struct CListPtrIter<V : From<CItemPtr>>{
    vec : *const Vec<ConstItem>,
    list_def : *const ListDefObj,
    root : *const RootDefObj,
    index : usize,
    phantom : PhantomData<*mut V>,
}
impl<V : From<CItemPtr>> Iterator for CListPtrIter<V>{
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        let vec = unsafe{ &*self.vec };
        if self.index < vec.len(){
            let index = self.index;
            self.index += 1;
            Some(V::from(CItemPtr::new(vec.get(index).unwrap(), self.list_def, self.root)))
        } else{
            None
        }

    }
}
impl<V : From<CItemPtr>> CListPtrIter<V>{
    pub fn new(vec : *const Vec<ConstItem>, list_def : *const ListDefObj, root : *const RootDefObj) -> CListPtrIter<V>{
        CListPtrIter { vec, list_def, root, index : 0, phantom : PhantomData }
    }
}
