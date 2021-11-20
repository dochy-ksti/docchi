#[cfg(test)] pub mod test{
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
	
		pub fn hoge_undef_null_undef_to_null(&self) -> Qv<i64>{
			let qv = root::get_int(self.ptr, "hogeUndefNullUndefToNull").unwrap();
			qv
		}
		pub fn hoge_undef_null_undef_to_null_def_val(&self) -> Qv<i64>{
			let qv = root::get_int_def(self.ptr, "hogeUndefNullUndefToNull").unwrap();
			qv
		}
		pub fn set_hoge_undef_null_undef_to_null(&mut self, hoge_undef_null_undef_to_null : Qv<i64>){
			root::set_int(self.ptr, "hogeUndefNullUndefToNull", hoge_undef_null_undef_to_null.into_qv());
		}
		pub fn hoge_int(&self) -> i64{
			let qv = root::get_int(self.ptr, "hogeInt").unwrap();
			qv.into_value().unwrap()
		}
		pub fn hoge_int_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "hogeInt").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_hoge_int(&mut self, hoge_int : i64){
			root::set_int(self.ptr, "hogeInt", Qv::Val(hoge_int));
		}
		pub fn hoge_float_array_def_val(&self) -> &Vec<f64>{
			let qv = root::get_float_array_def(self.ptr, "hogeFloatArray").unwrap();
			qv.into_value().unwrap()
		}
		pub fn hoge_float_array(&self) -> &Vec<f64>{
			let qv = root::get_immutable_float_array(self.ptr, "hogeFloatArray").unwrap();
			qv.into_value().unwrap()
		}
		pub fn hoge_float_array_mut(&mut self) -> &mut Vec<f64>{
			let qv = root::get_mutable_float_array(self.ptr, "hogeFloatArray").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_hoge_float_array(&mut self, hoge_float_array : Vec<f64>){
			root::set_float_array(self.ptr, "hogeFloatArray", Qv::Val(hoge_float_array));
		}
		pub fn hoge_bool(&self) -> bool{
			let qv = root::get_bool(self.ptr, "hogeBool").unwrap();
			qv.into_value().unwrap()
		}
		pub fn hoge_bool_def_val(&self) -> bool{
			let qv = root::get_bool_def(self.ptr, "hogeBool").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_hoge_bool(&mut self, hoge_bool : bool){
			root::set_bool(self.ptr, "hogeBool", Qv::Val(hoge_bool));
		}
		pub fn hoge_binary_def_val(&self) -> &Vec<u8>{
			let qv = root::get_binary_def(self.ptr, "hogeBinary").unwrap();
			qv.into_value().unwrap()
		}
		pub fn hoge_binary(&self) -> &Vec<u8>{
			let qv = root::get_immutable_binary(self.ptr, "hogeBinary").unwrap();
			qv.into_value().unwrap()
		}
		pub fn hoge_binary_mut(&mut self) -> &mut Vec<u8>{
			let qv = root::get_mutable_binary(self.ptr, "hogeBinary").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_hoge_binary(&mut self, hoge_binary : Vec<u8>){
			root::set_binary(self.ptr, "hogeBinary", Qv::Val(hoge_binary));
		}
		pub fn hoge_undef_null_undef(&self) -> Qv<i64>{
			let qv = root::get_int(self.ptr, "hogeUndefNullUndef").unwrap();
			qv
		}
		pub fn hoge_undef_null_undef_def_val(&self) -> Qv<i64>{
			let qv = root::get_int_def(self.ptr, "hogeUndefNullUndef").unwrap();
			qv
		}
		pub fn set_hoge_undef_null_undef(&mut self, hoge_undef_null_undef : Qv<i64>){
			root::set_int(self.ptr, "hogeUndefNullUndef", hoge_undef_null_undef.into_qv());
		}
		pub fn hoge_undef_null_to_null(&self) -> Qv<i64>{
			let qv = root::get_int(self.ptr, "hogeUndefNullToNull").unwrap();
			qv
		}
		pub fn hoge_undef_null_to_null_def_val(&self) -> Qv<i64>{
			let qv = root::get_int_def(self.ptr, "hogeUndefNullToNull").unwrap();
			qv
		}
		pub fn set_hoge_undef_null_to_null(&mut self, hoge_undef_null_to_null : Qv<i64>){
			root::set_int(self.ptr, "hogeUndefNullToNull", hoge_undef_null_to_null.into_qv());
		}
		pub fn hoge_string_def_val(&self) -> &String{
			let qv = root::get_str_def(self.ptr, "hogeString").unwrap();
			qv.into_value().unwrap()
		}
		pub fn hoge_string(&self) -> &String{
			let qv = root::get_immutable_str(self.ptr, "hogeString").unwrap();
			qv.into_value().unwrap()
		}
		pub fn hoge_string_mut(&mut self) -> &mut String{
			let qv = root::get_mutable_str(self.ptr, "hogeString").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_hoge_string(&mut self, hoge_string : String){
			root::set_str(self.ptr, "hogeString", Qv::Val(hoge_string));
		}
		pub fn hoge_undef_null_null_to_undef(&self) -> Qv<i64>{
			let qv = root::get_int(self.ptr, "hogeUndefNullNullToUndef").unwrap();
			qv
		}
		pub fn hoge_undef_null_null_to_undef_def_val(&self) -> Qv<i64>{
			let qv = root::get_int_def(self.ptr, "hogeUndefNullNullToUndef").unwrap();
			qv
		}
		pub fn set_hoge_undef_null_null_to_undef(&mut self, hoge_undef_null_null_to_undef : Qv<i64>){
			root::set_int(self.ptr, "hogeUndefNullNullToUndef", hoge_undef_null_null_to_undef.into_qv());
		}
		pub fn hoge_int_hatena_to_null(&self) -> NullOr<i64>{
			let qv = root::get_int(self.ptr, "hogeIntHatenaToNull").unwrap();
			NullOr::from_qv(qv).unwrap()
		}
		pub fn hoge_int_hatena_to_null_def_val(&self) -> NullOr<i64>{
			let qv = root::get_int_def(self.ptr, "hogeIntHatenaToNull").unwrap();
			NullOr::from_qv(qv).unwrap()
		}
		pub fn set_hoge_int_hatena_to_null(&mut self, hoge_int_hatena_to_null : NullOr<i64>){
			root::set_int(self.ptr, "hogeIntHatenaToNull", hoge_int_hatena_to_null.into_qv());
		}
		pub fn hoge_int_hatena_null(&self) -> NullOr<i64>{
			let qv = root::get_int(self.ptr, "hogeIntHatenaNull").unwrap();
			NullOr::from_qv(qv).unwrap()
		}
		pub fn hoge_int_hatena_null_def_val(&self) -> NullOr<i64>{
			let qv = root::get_int_def(self.ptr, "hogeIntHatenaNull").unwrap();
			NullOr::from_qv(qv).unwrap()
		}
		pub fn set_hoge_int_hatena_null(&mut self, hoge_int_hatena_null : NullOr<i64>){
			root::set_int(self.ptr, "hogeIntHatenaNull", hoge_int_hatena_null.into_qv());
		}
		pub fn hoge_undefined_undef(&self) -> UndefOr<i64>{
			let qv = root::get_int(self.ptr, "hogeUndefinedUndef").unwrap();
			UndefOr::from_qv(qv).unwrap()
		}
		pub fn hoge_undefined_undef_def_val(&self) -> UndefOr<i64>{
			let qv = root::get_int_def(self.ptr, "hogeUndefinedUndef").unwrap();
			UndefOr::from_qv(qv).unwrap()
		}
		pub fn set_hoge_undefined_undef(&mut self, hoge_undefined_undef : UndefOr<i64>){
			root::set_int(self.ptr, "hogeUndefinedUndef", hoge_undefined_undef.into_qv());
		}
		pub fn hoge_undef_null_null(&self) -> Qv<i64>{
			let qv = root::get_int(self.ptr, "hogeUndefNullNull").unwrap();
			qv
		}
		pub fn hoge_undef_null_null_def_val(&self) -> Qv<i64>{
			let qv = root::get_int_def(self.ptr, "hogeUndefNullNull").unwrap();
			qv
		}
		pub fn set_hoge_undef_null_null(&mut self, hoge_undef_null_null : Qv<i64>){
			root::set_int(self.ptr, "hogeUndefNullNull", hoge_undef_null_null.into_qv());
		}
		pub fn hoge_float(&self) -> f64{
			let qv = root::get_float(self.ptr, "hogeFloat").unwrap();
			qv.into_value().unwrap()
		}
		pub fn hoge_float_def_val(&self) -> f64{
			let qv = root::get_float_def(self.ptr, "hogeFloat").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_hoge_float(&mut self, hoge_float : f64){
			root::set_float(self.ptr, "hogeFloat", Qv::Val(hoge_float));
		}
		pub fn hoge_int_array_def_val(&self) -> &Vec<i64>{
			let qv = root::get_int_array_def(self.ptr, "hogeIntArray").unwrap();
			qv.into_value().unwrap()
		}
		pub fn hoge_int_array(&self) -> &Vec<i64>{
			let qv = root::get_immutable_int_array(self.ptr, "hogeIntArray").unwrap();
			qv.into_value().unwrap()
		}
		pub fn hoge_int_array_mut(&mut self) -> &mut Vec<i64>{
			let qv = root::get_mutable_int_array(self.ptr, "hogeIntArray").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_hoge_int_array(&mut self, hoge_int_array : Vec<i64>){
			root::set_int_array(self.ptr, "hogeIntArray", Qv::Val(hoge_int_array));
		}
		pub fn hoge_int_hatena(&self) -> NullOr<i64>{
			let qv = root::get_int(self.ptr, "hogeIntHatena").unwrap();
			NullOr::from_qv(qv).unwrap()
		}
		pub fn hoge_int_hatena_def_val(&self) -> NullOr<i64>{
			let qv = root::get_int_def(self.ptr, "hogeIntHatena").unwrap();
			NullOr::from_qv(qv).unwrap()
		}
		pub fn set_hoge_int_hatena(&mut self, hoge_int_hatena : NullOr<i64>){
			root::set_int(self.ptr, "hogeIntHatena", hoge_int_hatena.into_qv());
		}
		pub fn hoge_undefined(&self) -> UndefOr<i64>{
			let qv = root::get_int(self.ptr, "hogeUndefined").unwrap();
			UndefOr::from_qv(qv).unwrap()
		}
		pub fn hoge_undefined_def_val(&self) -> UndefOr<i64>{
			let qv = root::get_int_def(self.ptr, "hogeUndefined").unwrap();
			UndefOr::from_qv(qv).unwrap()
		}
		pub fn set_hoge_undefined(&mut self, hoge_undefined : UndefOr<i64>){
			root::set_int(self.ptr, "hogeUndefined", hoge_undefined.into_qv());
		}
		pub fn hoge_undefined_to_undef(&self) -> UndefOr<i64>{
			let qv = root::get_int(self.ptr, "hogeUndefinedToUndef").unwrap();
			UndefOr::from_qv(qv).unwrap()
		}
		pub fn hoge_undefined_to_undef_def_val(&self) -> UndefOr<i64>{
			let qv = root::get_int_def(self.ptr, "hogeUndefinedToUndef").unwrap();
			UndefOr::from_qv(qv).unwrap()
		}
		pub fn set_hoge_undefined_to_undef(&mut self, hoge_undefined_to_undef : UndefOr<i64>){
			root::set_int(self.ptr, "hogeUndefinedToUndef", hoge_undefined_to_undef.into_qv());
		}
		pub fn hoge_undef_null_to_undef(&self) -> Qv<i64>{
			let qv = root::get_int(self.ptr, "hogeUndefNullToUndef").unwrap();
			qv
		}
		pub fn hoge_undef_null_to_undef_def_val(&self) -> Qv<i64>{
			let qv = root::get_int_def(self.ptr, "hogeUndefNullToUndef").unwrap();
			qv
		}
		pub fn set_hoge_undef_null_to_undef(&mut self, hoge_undef_null_to_undef : Qv<i64>){
			root::set_int(self.ptr, "hogeUndefNullToUndef", hoge_undef_null_to_undef.into_qv());
		}
	}
	
}
