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

	pub fn mlist(&self) -> MListConst<MlistMItem>{
		let mil = root::get_mlist_const(self.ptr, "mlist").unwrap().unwrap();
		MListConst::new(mil, self)
	}
	pub fn mlist_mut(&mut self) -> MListMut<MlistMItem>{
		let mil = root::get_mlist_mut(self.ptr, "mlist").unwrap();
		MListMut::new(mil, self)
	}
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MlistMItem {
	ptr : MItemPtr,
}
impl From<MItemPtr> for MlistMItem {
	fn from(ptr : MItemPtr) -> Self {
		Self{ ptr }
	}
}
impl MlistMItem {
	pub fn val(&self) -> i64{
		let qv = mitem::get_int(self.ptr, "val").unwrap();
		qv.into_value().unwrap()
	}
	pub fn val_def_val(&self) -> i64{
		let qv = mitem::get_int_def(self.ptr, "val").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_val(&mut self, val : i64){
		mitem::set_int(self.ptr, "val", Qv::Val(val));
	}
	pub fn name_def_val(&self) -> &String{
		let qv = mitem::get_str_def(self.ptr, "name").unwrap();
		qv.into_value().unwrap()
	}
	pub fn name(&self) -> &String{
		let qv = mitem::get_immutable_str(self.ptr, "name").unwrap();
		qv.into_value().unwrap()
	}
	pub fn name_mut(&mut self) -> &mut String{
		let qv = mitem::get_mutable_str(self.ptr, "name").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_name(&mut self, name : String){
		mitem::set_str(self.ptr, "name", Qv::Val(name));
	}
	
	
}

