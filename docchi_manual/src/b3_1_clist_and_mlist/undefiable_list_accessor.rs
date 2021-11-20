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

	pub fn list(&self) -> Option<MListConst<ListMItem>>{
		let mil = root::get_mlist_const(self.ptr, "list").unwrap();
		mil.map(move |p| MListConst::new(p, self))
	}
	pub fn list_mut(&mut self) -> MListMut<ListMItem>{
		let mil = root::get_mlist_mut(self.ptr, "list").unwrap();
		MListMut::new(mil, self)
	}
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ListMItem {
	ptr : MItemPtr,
}
impl From<MItemPtr> for ListMItem {
	fn from(ptr : MItemPtr) -> Self {
		Self{ ptr }
	}
}
impl ListMItem {
	pub fn v(&self) -> i64{
		let qv = mitem::get_int(self.ptr, "v").unwrap();
		qv.into_value().unwrap()
	}
	pub fn v_def_val(&self) -> i64{
		let qv = mitem::get_int_def(self.ptr, "v").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_v(&mut self, v : i64){
		mitem::set_int(self.ptr, "v", Qv::Val(v));
	}
	
	
}

