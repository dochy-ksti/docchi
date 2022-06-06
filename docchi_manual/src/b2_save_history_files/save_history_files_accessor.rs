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

	pub fn d3_def_val(&self) -> &String{
		let qv = root::get_str_def(self.ptr, "d3").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d3(&self) -> &String{
		let qv = root::get_immutable_str(self.ptr, "d3").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d3_mut(&mut self) -> &mut String{
		let qv = root::get_mutable_str(self.ptr, "d3").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_d3(&mut self, d3 : String){
		root::set_str(self.ptr, "d3", Qv::Val(d3));
	}
	pub fn d4_def_val(&self) -> &String{
		let qv = root::get_str_def(self.ptr, "d4").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d4(&self) -> &String{
		let qv = root::get_immutable_str(self.ptr, "d4").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d4_mut(&mut self) -> &mut String{
		let qv = root::get_mutable_str(self.ptr, "d4").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_d4(&mut self, d4 : String){
		root::set_str(self.ptr, "d4", Qv::Val(d4));
	}
	pub fn d5_def_val(&self) -> &String{
		let qv = root::get_str_def(self.ptr, "d5").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d5(&self) -> &String{
		let qv = root::get_immutable_str(self.ptr, "d5").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d5_mut(&mut self) -> &mut String{
		let qv = root::get_mutable_str(self.ptr, "d5").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_d5(&mut self, d5 : String){
		root::set_str(self.ptr, "d5", Qv::Val(d5));
	}
	pub fn d6_def_val(&self) -> &String{
		let qv = root::get_str_def(self.ptr, "d6").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d6(&self) -> &String{
		let qv = root::get_immutable_str(self.ptr, "d6").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d6_mut(&mut self) -> &mut String{
		let qv = root::get_mutable_str(self.ptr, "d6").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_d6(&mut self, d6 : String){
		root::set_str(self.ptr, "d6", Qv::Val(d6));
	}
	pub fn d0_def_val(&self) -> &String{
		let qv = root::get_str_def(self.ptr, "d0").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d0(&self) -> &String{
		let qv = root::get_immutable_str(self.ptr, "d0").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d0_mut(&mut self) -> &mut String{
		let qv = root::get_mutable_str(self.ptr, "d0").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_d0(&mut self, d0 : String){
		root::set_str(self.ptr, "d0", Qv::Val(d0));
	}
	pub fn d7_def_val(&self) -> &String{
		let qv = root::get_str_def(self.ptr, "d7").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d7(&self) -> &String{
		let qv = root::get_immutable_str(self.ptr, "d7").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d7_mut(&mut self) -> &mut String{
		let qv = root::get_mutable_str(self.ptr, "d7").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_d7(&mut self, d7 : String){
		root::set_str(self.ptr, "d7", Qv::Val(d7));
	}
	pub fn d8_def_val(&self) -> &String{
		let qv = root::get_str_def(self.ptr, "d8").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d8(&self) -> &String{
		let qv = root::get_immutable_str(self.ptr, "d8").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d8_mut(&mut self) -> &mut String{
		let qv = root::get_mutable_str(self.ptr, "d8").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_d8(&mut self, d8 : String){
		root::set_str(self.ptr, "d8", Qv::Val(d8));
	}
	pub fn d1_def_val(&self) -> &String{
		let qv = root::get_str_def(self.ptr, "d1").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d1(&self) -> &String{
		let qv = root::get_immutable_str(self.ptr, "d1").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d1_mut(&mut self) -> &mut String{
		let qv = root::get_mutable_str(self.ptr, "d1").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_d1(&mut self, d1 : String){
		root::set_str(self.ptr, "d1", Qv::Val(d1));
	}
	pub fn d2_def_val(&self) -> &String{
		let qv = root::get_str_def(self.ptr, "d2").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d2(&self) -> &String{
		let qv = root::get_immutable_str(self.ptr, "d2").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d2_mut(&mut self) -> &mut String{
		let qv = root::get_mutable_str(self.ptr, "d2").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_d2(&mut self, d2 : String){
		root::set_str(self.ptr, "d2", Qv::Val(d2));
	}
	pub fn d9_def_val(&self) -> &String{
		let qv = root::get_str_def(self.ptr, "d9").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d9(&self) -> &String{
		let qv = root::get_immutable_str(self.ptr, "d9").unwrap();
		qv.into_value().unwrap()
	}
	pub fn d9_mut(&mut self) -> &mut String{
		let qv = root::get_mutable_str(self.ptr, "d9").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_d9(&mut self, d9 : String){
		root::set_str(self.ptr, "d9", Qv::Val(d9));
	}
}
