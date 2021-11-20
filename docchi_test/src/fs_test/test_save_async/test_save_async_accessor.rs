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

	pub fn data0(&self) -> i64{
		let qv = root::get_int(self.ptr, "data0").unwrap();
		qv.into_value().unwrap()
	}
	pub fn data0_def_val(&self) -> i64{
		let qv = root::get_int_def(self.ptr, "data0").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_data0(&mut self, data0 : i64){
		root::set_int(self.ptr, "data0", Qv::Val(data0));
	}
}
