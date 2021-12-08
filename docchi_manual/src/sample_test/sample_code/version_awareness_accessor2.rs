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

	pub fn old_value(&self) -> NullOr<i64>{
		let qv = root::get_int(self.ptr, "oldValue").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn old_value_def_val(&self) -> NullOr<i64>{
		let qv = root::get_int_def(self.ptr, "oldValue").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn set_old_value(&mut self, old_value : NullOr<i64>){
		root::set_int(self.ptr, "oldValue", old_value.into_qv());
	}
	pub fn new_value2(&self) -> UndefOr<i64>{
		let qv = root::get_int(self.ptr, "newValue2").unwrap();
		UndefOr::from_qv(qv).unwrap()
	}
	pub fn new_value2_def_val(&self) -> UndefOr<i64>{
		let qv = root::get_int_def(self.ptr, "newValue2").unwrap();
		UndefOr::from_qv(qv).unwrap()
	}
	pub fn set_new_value2(&mut self, new_value2 : UndefOr<i64>){
		root::set_int(self.ptr, "newValue2", new_value2.into_qv());
	}
	pub fn new_value(&self) -> Qv<i64>{
		let qv = root::get_int(self.ptr, "newValue").unwrap();
		qv
	}
	pub fn new_value_def_val(&self) -> Qv<i64>{
		let qv = root::get_int_def(self.ptr, "newValue").unwrap();
		qv
	}
	pub fn set_new_value(&mut self, new_value : Qv<i64>){
		root::set_int(self.ptr, "newValue", new_value.into_qv());
	}
}
