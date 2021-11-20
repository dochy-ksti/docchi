use docchi::core::intf::*;
use docchi::core::structs::*;
unsafe impl Send for RootIntf {}
unsafe impl Sync for RootIntf {}
#[derive(Debug)]
pub struct RootIntf{
    root : Box<RootObject>,
    ptr : RootObjectPtr,
}
impl RootIntf{
    pub fn new(obj : RootObject) -> RootIntf{
		let mut root = Box::new(obj);
		let ptr = RootObjectPtr::new(root.as_mut());
		RootIntf { root, ptr }
	}
    pub fn root_obj_ref(&self) -> &RootObject{ self.root.as_ref() }
    pub fn root_obj_ref_mut(&mut self) -> &mut RootObject{ self.root.as_mut() }

	pub fn list(&self) -> CListConst<ListCItem>{
		CListConst::new(root::get_clist(self.ptr, "list").unwrap(), self)
	}
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ListCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for ListCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl ListCItem {
	pub fn foo(&self) -> i64{
		let qv = citem::get_int(self.ptr, "foo").unwrap();
		qv.into_value().unwrap()
	}
	pub fn foo_def_val(&self) -> i64{
		let qv = citem::get_int_def(self.ptr, "foo").unwrap();
		qv.into_value().unwrap()
	}
	
}

