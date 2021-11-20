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
    pub fn deconstruct(self) -> RootObject{ *self.root }

	pub fn clist(&self) -> CListConst<ClistCItem>{
		CListConst::new(root::get_clist(self.ptr, "clist").unwrap(), self)
	}
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ClistCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for ClistCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl ClistCItem {
	pub fn val(&self) -> i64{
		let qv = citem::get_int(self.ptr, "val").unwrap();
		qv.into_value().unwrap()
	}
	pub fn val_def_val(&self) -> i64{
		let qv = citem::get_int_def(self.ptr, "val").unwrap();
		qv.into_value().unwrap()
	}
	pub fn name_def_val(&self) -> &String{
		let qv = citem::get_str_def(self.ptr, "name").unwrap();
		qv.into_value().unwrap()
	}
	pub fn name(&self) -> &String{
		let qv = citem::get_immutable_str(self.ptr, "name").unwrap();
		qv.into_value().unwrap()
	}
	
}

