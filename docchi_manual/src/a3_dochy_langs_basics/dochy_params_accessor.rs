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

	pub fn nullable_str_def_val(&self) -> NullOr<&String>{
		let qv = root::get_str_def(self.ptr, "nullable_str").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn nullable_str(&self) -> NullOr<&String>{
		let qv = root::get_immutable_str(self.ptr, "nullable_str").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn nullable_str_mut(&mut self) -> NullOr<&mut String>{
		let qv = root::get_mutable_str(self.ptr, "nullable_str").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn set_nullable_str(&mut self, nullable_str : NullOr<String>){
		root::set_str(self.ptr, "nullable_str", nullable_str.into_qv());
	}
	pub fn nullable_bool(&self) -> NullOr<bool>{
		let qv = root::get_bool(self.ptr, "nullable_bool").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn nullable_bool_def_val(&self) -> NullOr<bool>{
		let qv = root::get_bool_def(self.ptr, "nullable_bool").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn set_nullable_bool(&mut self, nullable_bool : NullOr<bool>){
		root::set_bool(self.ptr, "nullable_bool", nullable_bool.into_qv());
	}
	pub fn undef_nullable_int(&self) -> Qv<i64>{
		let qv = root::get_int(self.ptr, "undef_nullable_int").unwrap();
		qv
	}
	pub fn undef_nullable_int_def_val(&self) -> Qv<i64>{
		let qv = root::get_int_def(self.ptr, "undef_nullable_int").unwrap();
		qv
	}
	pub fn set_undef_nullable_int(&mut self, undef_nullable_int : Qv<i64>){
		root::set_int(self.ptr, "undef_nullable_int", undef_nullable_int.into_qv());
	}
	pub fn nullable_binary_def_val(&self) -> NullOr<&Vec<u8>>{
		let qv = root::get_binary_def(self.ptr, "nullable_binary").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn nullable_binary(&self) -> NullOr<&Vec<u8>>{
		let qv = root::get_immutable_binary(self.ptr, "nullable_binary").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn nullable_binary_mut(&mut self) -> NullOr<&mut Vec<u8>>{
		let qv = root::get_mutable_binary(self.ptr, "nullable_binary").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn set_nullable_binary(&mut self, nullable_binary : NullOr<Vec<u8>>){
		root::set_binary(self.ptr, "nullable_binary", nullable_binary.into_qv());
	}
	pub fn bool_value(&self) -> bool{
		let qv = root::get_bool(self.ptr, "bool_value").unwrap();
		qv.into_value().unwrap()
	}
	pub fn bool_value_def_val(&self) -> bool{
		let qv = root::get_bool_def(self.ptr, "bool_value").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_bool_value(&mut self, bool_value : bool){
		root::set_bool(self.ptr, "bool_value", Qv::Val(bool_value));
	}
	pub fn nullable_int_array_def_val(&self) -> NullOr<&Vec<i64>>{
		let qv = root::get_int_array_def(self.ptr, "nullable_int_array").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn nullable_int_array(&self) -> NullOr<&Vec<i64>>{
		let qv = root::get_immutable_int_array(self.ptr, "nullable_int_array").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn nullable_int_array_mut(&mut self) -> NullOr<&mut Vec<i64>>{
		let qv = root::get_mutable_int_array(self.ptr, "nullable_int_array").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn set_nullable_int_array(&mut self, nullable_int_array : NullOr<Vec<i64>>){
		root::set_int_array(self.ptr, "nullable_int_array", nullable_int_array.into_qv());
	}
	pub fn undefiable_int(&self) -> UndefOr<i64>{
		let qv = root::get_int(self.ptr, "undefiable_int").unwrap();
		UndefOr::from_qv(qv).unwrap()
	}
	pub fn undefiable_int_def_val(&self) -> UndefOr<i64>{
		let qv = root::get_int_def(self.ptr, "undefiable_int").unwrap();
		UndefOr::from_qv(qv).unwrap()
	}
	pub fn set_undefiable_int(&mut self, undefiable_int : UndefOr<i64>){
		root::set_int(self.ptr, "undefiable_int", undefiable_int.into_qv());
	}
	pub fn nullable_float_array_def_val(&self) -> NullOr<&Vec<f64>>{
		let qv = root::get_float_array_def(self.ptr, "nullable_float_array").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn nullable_float_array(&self) -> NullOr<&Vec<f64>>{
		let qv = root::get_immutable_float_array(self.ptr, "nullable_float_array").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn nullable_float_array_mut(&mut self) -> NullOr<&mut Vec<f64>>{
		let qv = root::get_mutable_float_array(self.ptr, "nullable_float_array").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn set_nullable_float_array(&mut self, nullable_float_array : NullOr<Vec<f64>>){
		root::set_float_array(self.ptr, "nullable_float_array", nullable_float_array.into_qv());
	}
	pub fn string_value_def_val(&self) -> &String{
		let qv = root::get_str_def(self.ptr, "string_value").unwrap();
		qv.into_value().unwrap()
	}
	pub fn string_value(&self) -> &String{
		let qv = root::get_immutable_str(self.ptr, "string_value").unwrap();
		qv.into_value().unwrap()
	}
	pub fn string_value_mut(&mut self) -> &mut String{
		let qv = root::get_mutable_str(self.ptr, "string_value").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_string_value(&mut self, string_value : String){
		root::set_str(self.ptr, "string_value", Qv::Val(string_value));
	}
	pub fn float_value(&self) -> f64{
		let qv = root::get_float(self.ptr, "float_value").unwrap();
		qv.into_value().unwrap()
	}
	pub fn float_value_def_val(&self) -> f64{
		let qv = root::get_float_def(self.ptr, "float_value").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_float_value(&mut self, float_value : f64){
		root::set_float(self.ptr, "float_value", Qv::Val(float_value));
	}
	pub fn nullable_int(&self) -> NullOr<i64>{
		let qv = root::get_int(self.ptr, "nullable_int").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn nullable_int_def_val(&self) -> NullOr<i64>{
		let qv = root::get_int_def(self.ptr, "nullable_int").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn set_nullable_int(&mut self, nullable_int : NullOr<i64>){
		root::set_int(self.ptr, "nullable_int", nullable_int.into_qv());
	}
	pub fn int_array_def_val(&self) -> &Vec<i64>{
		let qv = root::get_int_array_def(self.ptr, "int_array").unwrap();
		qv.into_value().unwrap()
	}
	pub fn int_array(&self) -> &Vec<i64>{
		let qv = root::get_immutable_int_array(self.ptr, "int_array").unwrap();
		qv.into_value().unwrap()
	}
	pub fn int_array_mut(&mut self) -> &mut Vec<i64>{
		let qv = root::get_mutable_int_array(self.ptr, "int_array").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_int_array(&mut self, int_array : Vec<i64>){
		root::set_int_array(self.ptr, "int_array", Qv::Val(int_array));
	}
	pub fn nullable_int2(&self) -> NullOr<i64>{
		let qv = root::get_int(self.ptr, "nullable_int2").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn nullable_int2_def_val(&self) -> NullOr<i64>{
		let qv = root::get_int_def(self.ptr, "nullable_int2").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn set_nullable_int2(&mut self, nullable_int2 : NullOr<i64>){
		root::set_int(self.ptr, "nullable_int2", nullable_int2.into_qv());
	}
	pub fn float_array_def_val(&self) -> &Vec<f64>{
		let qv = root::get_float_array_def(self.ptr, "float_array").unwrap();
		qv.into_value().unwrap()
	}
	pub fn float_array(&self) -> &Vec<f64>{
		let qv = root::get_immutable_float_array(self.ptr, "float_array").unwrap();
		qv.into_value().unwrap()
	}
	pub fn float_array_mut(&mut self) -> &mut Vec<f64>{
		let qv = root::get_mutable_float_array(self.ptr, "float_array").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_float_array(&mut self, float_array : Vec<f64>){
		root::set_float_array(self.ptr, "float_array", Qv::Val(float_array));
	}
	pub fn binary_def_val(&self) -> &Vec<u8>{
		let qv = root::get_binary_def(self.ptr, "binary").unwrap();
		qv.into_value().unwrap()
	}
	pub fn binary(&self) -> &Vec<u8>{
		let qv = root::get_immutable_binary(self.ptr, "binary").unwrap();
		qv.into_value().unwrap()
	}
	pub fn binary_mut(&mut self) -> &mut Vec<u8>{
		let qv = root::get_mutable_binary(self.ptr, "binary").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_binary(&mut self, binary : Vec<u8>){
		root::set_binary(self.ptr, "binary", Qv::Val(binary));
	}
	pub fn nullable_float(&self) -> NullOr<f64>{
		let qv = root::get_float(self.ptr, "nullable_float").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn nullable_float_def_val(&self) -> NullOr<f64>{
		let qv = root::get_float_def(self.ptr, "nullable_float").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn set_nullable_float(&mut self, nullable_float : NullOr<f64>){
		root::set_float(self.ptr, "nullable_float", nullable_float.into_qv());
	}
	pub fn int_value(&self) -> i64{
		let qv = root::get_int(self.ptr, "int_value").unwrap();
		qv.into_value().unwrap()
	}
	pub fn int_value_def_val(&self) -> i64{
		let qv = root::get_int_def(self.ptr, "int_value").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_int_value(&mut self, int_value : i64){
		root::set_int(self.ptr, "int_value", Qv::Val(int_value));
	}
}
