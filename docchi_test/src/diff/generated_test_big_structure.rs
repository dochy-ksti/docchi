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
	
		pub fn mem54(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem54").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem54_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem54").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem54(&mut self, mem54 : i64){
			root::set_int(self.ptr, "mem54", Qv::Val(mem54));
		}
		pub fn mem27(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem27").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem27_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem27").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem27(&mut self, mem27 : i64){
			root::set_int(self.ptr, "mem27", Qv::Val(mem27));
		}
		pub fn mem36(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem36").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem36_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem36").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem36(&mut self, mem36 : i64){
			root::set_int(self.ptr, "mem36", Qv::Val(mem36));
		}
		pub fn mem6(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem6").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem6_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem6").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem6(&mut self, mem6 : i64){
			root::set_int(self.ptr, "mem6", Qv::Val(mem6));
		}
		pub fn mem108(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem108").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem108_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem108").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem108(&mut self, mem108 : i64){
			root::set_int(self.ptr, "mem108", Qv::Val(mem108));
		}
		pub fn mem93(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem93").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem93_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem93").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem93(&mut self, mem93 : i64){
			root::set_int(self.ptr, "mem93", Qv::Val(mem93));
		}
		pub fn mem125(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem125").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem125_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem125").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem125(&mut self, mem125 : i64){
			root::set_int(self.ptr, "mem125", Qv::Val(mem125));
		}
		pub fn mem79(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem79").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem79_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem79").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem79(&mut self, mem79 : i64){
			root::set_int(self.ptr, "mem79", Qv::Val(mem79));
		}
		pub fn mem30(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem30").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem30_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem30").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem30(&mut self, mem30 : i64){
			root::set_int(self.ptr, "mem30", Qv::Val(mem30));
		}
		pub fn mem123(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem123").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem123_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem123").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem123(&mut self, mem123 : i64){
			root::set_int(self.ptr, "mem123", Qv::Val(mem123));
		}
		pub fn mem99(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem99").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem99_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem99").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem99(&mut self, mem99 : i64){
			root::set_int(self.ptr, "mem99", Qv::Val(mem99));
		}
		pub fn mem62(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem62").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem62_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem62").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem62(&mut self, mem62 : i64){
			root::set_int(self.ptr, "mem62", Qv::Val(mem62));
		}
		pub fn mem73(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem73").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem73_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem73").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem73(&mut self, mem73 : i64){
			root::set_int(self.ptr, "mem73", Qv::Val(mem73));
		}
		pub fn mem110(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem110").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem110_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem110").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem110(&mut self, mem110 : i64){
			root::set_int(self.ptr, "mem110", Qv::Val(mem110));
		}
		pub fn mem105(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem105").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem105_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem105").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem105(&mut self, mem105 : i64){
			root::set_int(self.ptr, "mem105", Qv::Val(mem105));
		}
		pub fn mem47(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem47").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem47_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem47").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem47(&mut self, mem47 : i64){
			root::set_int(self.ptr, "mem47", Qv::Val(mem47));
		}
		pub fn mem75(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem75").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem75_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem75").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem75(&mut self, mem75 : i64){
			root::set_int(self.ptr, "mem75", Qv::Val(mem75));
		}
		pub fn mem11(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem11").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem11_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem11").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem11(&mut self, mem11 : i64){
			root::set_int(self.ptr, "mem11", Qv::Val(mem11));
		}
		pub fn mem83(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem83").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem83_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem83").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem83(&mut self, mem83 : i64){
			root::set_int(self.ptr, "mem83", Qv::Val(mem83));
		}
		pub fn mem41(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem41").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem41_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem41").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem41(&mut self, mem41 : i64){
			root::set_int(self.ptr, "mem41", Qv::Val(mem41));
		}
		pub fn mem55(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem55").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem55_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem55").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem55(&mut self, mem55 : i64){
			root::set_int(self.ptr, "mem55", Qv::Val(mem55));
		}
		pub fn mem26(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem26").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem26_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem26").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem26(&mut self, mem26 : i64){
			root::set_int(self.ptr, "mem26", Qv::Val(mem26));
		}
		pub fn mem37(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem37").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem37_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem37").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem37(&mut self, mem37 : i64){
			root::set_int(self.ptr, "mem37", Qv::Val(mem37));
		}
		pub fn mem9(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem9").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem9_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem9").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem9(&mut self, mem9 : i64){
			root::set_int(self.ptr, "mem9", Qv::Val(mem9));
		}
		pub fn mem85(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem85").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem85_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem85").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem85(&mut self, mem85 : i64){
			root::set_int(self.ptr, "mem85", Qv::Val(mem85));
		}
		pub fn mem90(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem90").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem90_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem90").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem90(&mut self, mem90 : i64){
			root::set_int(self.ptr, "mem90", Qv::Val(mem90));
		}
		pub fn mem109(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem109").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem109_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem109").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem109(&mut self, mem109 : i64){
			root::set_int(self.ptr, "mem109", Qv::Val(mem109));
		}
		pub fn mem31(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem31").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem31_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem31").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem31(&mut self, mem31 : i64){
			root::set_int(self.ptr, "mem31", Qv::Val(mem31));
		}
		pub fn mem3(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem3").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem3_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem3").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem3(&mut self, mem3 : i64){
			root::set_int(self.ptr, "mem3", Qv::Val(mem3));
		}
		pub fn mem120(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem120").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem120_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem120").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem120(&mut self, mem120 : i64){
			root::set_int(self.ptr, "mem120", Qv::Val(mem120));
		}
		pub fn mem61(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem61").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem61_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem61").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem61(&mut self, mem61 : i64){
			root::set_int(self.ptr, "mem61", Qv::Val(mem61));
		}
		pub fn mem18(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem18").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem18_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem18").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem18(&mut self, mem18 : i64){
			root::set_int(self.ptr, "mem18", Qv::Val(mem18));
		}
		pub fn mem117(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem117").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem117_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem117").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem117(&mut self, mem117 : i64){
			root::set_int(self.ptr, "mem117", Qv::Val(mem117));
		}
		pub fn mem106(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem106").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem106_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem106").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem106(&mut self, mem106 : i64){
			root::set_int(self.ptr, "mem106", Qv::Val(mem106));
		}
		pub fn mem46(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem46").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem46_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem46").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem46(&mut self, mem46 : i64){
			root::set_int(self.ptr, "mem46", Qv::Val(mem46));
		}
		pub fn mem67(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem67").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem67_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem67").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem67(&mut self, mem67 : i64){
			root::set_int(self.ptr, "mem67", Qv::Val(mem67));
		}
		pub fn mem76(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem76").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem76_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem76").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem76(&mut self, mem76 : i64){
			root::set_int(self.ptr, "mem76", Qv::Val(mem76));
		}
		pub fn mem16(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem16").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem16_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem16").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem16(&mut self, mem16 : i64){
			root::set_int(self.ptr, "mem16", Qv::Val(mem16));
		}
		pub fn mem82(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem82").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem82_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem82").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem82(&mut self, mem82 : i64){
			root::set_int(self.ptr, "mem82", Qv::Val(mem82));
		}
		pub fn mem100(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem100").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem100_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem100").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem100(&mut self, mem100 : i64){
			root::set_int(self.ptr, "mem100", Qv::Val(mem100));
		}
		pub fn mem40(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem40").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem40_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem40").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem40(&mut self, mem40 : i64){
			root::set_int(self.ptr, "mem40", Qv::Val(mem40));
		}
		pub fn mem52(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem52").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem52_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem52").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem52(&mut self, mem52 : i64){
			root::set_int(self.ptr, "mem52", Qv::Val(mem52));
		}
		pub fn mem25(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem25").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem25_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem25").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem25(&mut self, mem25 : i64){
			root::set_int(self.ptr, "mem25", Qv::Val(mem25));
		}
		pub fn mem8(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem8").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem8_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem8").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem8(&mut self, mem8 : i64){
			root::set_int(self.ptr, "mem8", Qv::Val(mem8));
		}
		pub fn mem84(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem84").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem84_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem84").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem84(&mut self, mem84 : i64){
			root::set_int(self.ptr, "mem84", Qv::Val(mem84));
		}
		pub fn mem91(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem91").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem91_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem91").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem91(&mut self, mem91 : i64){
			root::set_int(self.ptr, "mem91", Qv::Val(mem91));
		}
		pub fn mem131(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem131").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem131_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem131").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem131(&mut self, mem131 : i64){
			root::set_int(self.ptr, "mem131", Qv::Val(mem131));
		}
		pub fn mem58(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem58").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem58_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem58").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem58(&mut self, mem58 : i64){
			root::set_int(self.ptr, "mem58", Qv::Val(mem58));
		}
		pub fn mem23(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem23").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem23_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem23").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem23(&mut self, mem23 : i64){
			root::set_int(self.ptr, "mem23", Qv::Val(mem23));
		}
		pub fn mem32(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem32").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem32_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem32").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem32(&mut self, mem32 : i64){
			root::set_int(self.ptr, "mem32", Qv::Val(mem32));
		}
		pub fn mem2(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem2").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem2_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem2").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem2(&mut self, mem2 : i64){
			root::set_int(self.ptr, "mem2", Qv::Val(mem2));
		}
		pub fn mem121(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem121").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem121_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem121").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem121(&mut self, mem121 : i64){
			root::set_int(self.ptr, "mem121", Qv::Val(mem121));
		}
		pub fn mem60(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem60").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem60_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem60").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem60(&mut self, mem60 : i64){
			root::set_int(self.ptr, "mem60", Qv::Val(mem60));
		}
		pub fn mem29(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem29").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem29_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem29").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem29(&mut self, mem29 : i64){
			root::set_int(self.ptr, "mem29", Qv::Val(mem29));
		}
		pub fn mem19(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem19").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem19_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem19").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem19(&mut self, mem19 : i64){
			root::set_int(self.ptr, "mem19", Qv::Val(mem19));
		}
		pub fn mem116(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem116").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem116_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem116").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem116(&mut self, mem116 : i64){
			root::set_int(self.ptr, "mem116", Qv::Val(mem116));
		}
		pub fn mem107(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem107").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem107_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem107").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem107(&mut self, mem107 : i64){
			root::set_int(self.ptr, "mem107", Qv::Val(mem107));
		}
		pub fn mem49(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem49").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem49_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem49").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem49(&mut self, mem49 : i64){
			root::set_int(self.ptr, "mem49", Qv::Val(mem49));
		}
		pub fn mem66(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem66").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem66_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem66").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem66(&mut self, mem66 : i64){
			root::set_int(self.ptr, "mem66", Qv::Val(mem66));
		}
		pub fn mem77(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem77").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem77_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem77").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem77(&mut self, mem77 : i64){
			root::set_int(self.ptr, "mem77", Qv::Val(mem77));
		}
		pub fn mem17(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem17").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem17_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem17").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem17(&mut self, mem17 : i64){
			root::set_int(self.ptr, "mem17", Qv::Val(mem17));
		}
		pub fn mem101(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem101").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem101_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem101").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem101(&mut self, mem101 : i64){
			root::set_int(self.ptr, "mem101", Qv::Val(mem101));
		}
		pub fn mem43(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem43").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem43_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem43").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem43(&mut self, mem43 : i64){
			root::set_int(self.ptr, "mem43", Qv::Val(mem43));
		}
		pub fn mem53(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem53").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem53_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem53").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem53(&mut self, mem53 : i64){
			root::set_int(self.ptr, "mem53", Qv::Val(mem53));
		}
		pub fn mem24(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem24").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem24_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem24").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem24(&mut self, mem24 : i64){
			root::set_int(self.ptr, "mem24", Qv::Val(mem24));
		}
		pub fn mem87(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem87").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem87_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem87").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem87(&mut self, mem87 : i64){
			root::set_int(self.ptr, "mem87", Qv::Val(mem87));
		}
		pub fn mem128(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem128").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem128_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem128").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem128(&mut self, mem128 : i64){
			root::set_int(self.ptr, "mem128", Qv::Val(mem128));
		}
		pub fn mem96(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem96").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem96_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem96").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem96(&mut self, mem96 : i64){
			root::set_int(self.ptr, "mem96", Qv::Val(mem96));
		}
		pub fn mem130(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem130").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem130_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem130").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem130(&mut self, mem130 : i64){
			root::set_int(self.ptr, "mem130", Qv::Val(mem130));
		}
		pub fn mem69(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem69").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem69_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem69").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem69(&mut self, mem69 : i64){
			root::set_int(self.ptr, "mem69", Qv::Val(mem69));
		}
		pub fn mem59(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem59").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem59_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem59").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem59(&mut self, mem59 : i64){
			root::set_int(self.ptr, "mem59", Qv::Val(mem59));
		}
		pub fn mem22(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem22").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem22_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem22").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem22(&mut self, mem22 : i64){
			root::set_int(self.ptr, "mem22", Qv::Val(mem22));
		}
		pub fn mem33(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem33").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem33_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem33").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem33(&mut self, mem33 : i64){
			root::set_int(self.ptr, "mem33", Qv::Val(mem33));
		}
		pub fn mem5(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem5").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem5_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem5").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem5(&mut self, mem5 : i64){
			root::set_int(self.ptr, "mem5", Qv::Val(mem5));
		}
		pub fn mem89(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem89").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem89_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem89").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem89(&mut self, mem89 : i64){
			root::set_int(self.ptr, "mem89", Qv::Val(mem89));
		}
		pub fn mem126(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem126").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem126_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem126").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem126(&mut self, mem126 : i64){
			root::set_int(self.ptr, "mem126", Qv::Val(mem126));
		}
		pub fn mem28(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem28").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem28_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem28").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem28(&mut self, mem28 : i64){
			root::set_int(self.ptr, "mem28", Qv::Val(mem28));
		}
		pub fn mem115(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem115").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem115_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem115").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem115(&mut self, mem115 : i64){
			root::set_int(self.ptr, "mem115", Qv::Val(mem115));
		}
		pub fn mem48(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem48").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem48_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem48").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem48(&mut self, mem48 : i64){
			root::set_int(self.ptr, "mem48", Qv::Val(mem48));
		}
		pub fn mem65(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem65").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem65_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem65").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem65(&mut self, mem65 : i64){
			root::set_int(self.ptr, "mem65", Qv::Val(mem65));
		}
		pub fn mem70(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem70").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem70_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem70").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem70(&mut self, mem70 : i64){
			root::set_int(self.ptr, "mem70", Qv::Val(mem70));
		}
		pub fn mem14(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem14").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem14_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem14").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem14(&mut self, mem14 : i64){
			root::set_int(self.ptr, "mem14", Qv::Val(mem14));
		}
		pub fn mem113(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem113").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem113_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem113").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem113(&mut self, mem113 : i64){
			root::set_int(self.ptr, "mem113", Qv::Val(mem113));
		}
		pub fn mem102(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem102").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem102_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem102").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem102(&mut self, mem102 : i64){
			root::set_int(self.ptr, "mem102", Qv::Val(mem102));
		}
		pub fn mem42(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem42").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem42_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem42").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem42(&mut self, mem42 : i64){
			root::set_int(self.ptr, "mem42", Qv::Val(mem42));
		}
		pub fn mem50(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem50").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem50_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem50").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem50(&mut self, mem50 : i64){
			root::set_int(self.ptr, "mem50", Qv::Val(mem50));
		}
		pub fn mem12(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem12").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem12_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem12").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem12(&mut self, mem12 : i64){
			root::set_int(self.ptr, "mem12", Qv::Val(mem12));
		}
		pub fn mem119(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem119").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem119_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem119").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem119(&mut self, mem119 : i64){
			root::set_int(self.ptr, "mem119", Qv::Val(mem119));
		}
		pub fn mem86(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem86").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem86_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem86").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem86(&mut self, mem86 : i64){
			root::set_int(self.ptr, "mem86", Qv::Val(mem86));
		}
		pub fn mem129(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem129").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem129_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem129").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem129(&mut self, mem129 : i64){
			root::set_int(self.ptr, "mem129", Qv::Val(mem129));
		}
		pub fn mem97(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem97").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem97_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem97").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem97(&mut self, mem97 : i64){
			root::set_int(self.ptr, "mem97", Qv::Val(mem97));
		}
		pub fn mem68(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem68").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem68_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem68").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem68(&mut self, mem68 : i64){
			root::set_int(self.ptr, "mem68", Qv::Val(mem68));
		}
		pub fn mem56(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem56").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem56_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem56").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem56(&mut self, mem56 : i64){
			root::set_int(self.ptr, "mem56", Qv::Val(mem56));
		}
		pub fn mem21(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem21").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem21_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem21").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem21(&mut self, mem21 : i64){
			root::set_int(self.ptr, "mem21", Qv::Val(mem21));
		}
		pub fn mem34(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem34").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem34_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem34").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem34(&mut self, mem34 : i64){
			root::set_int(self.ptr, "mem34", Qv::Val(mem34));
		}
		pub fn mem4(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem4").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem4_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem4").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem4(&mut self, mem4 : i64){
			root::set_int(self.ptr, "mem4", Qv::Val(mem4));
		}
		pub fn mem88(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem88").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem88_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem88").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem88(&mut self, mem88 : i64){
			root::set_int(self.ptr, "mem88", Qv::Val(mem88));
		}
		pub fn mem127(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem127").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem127_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem127").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem127(&mut self, mem127 : i64){
			root::set_int(self.ptr, "mem127", Qv::Val(mem127));
		}
		pub fn mem114(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem114").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem114_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem114").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem114(&mut self, mem114 : i64){
			root::set_int(self.ptr, "mem114", Qv::Val(mem114));
		}
		pub fn mem64(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem64").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem64_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem64").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem64(&mut self, mem64 : i64){
			root::set_int(self.ptr, "mem64", Qv::Val(mem64));
		}
		pub fn mem71(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem71").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem71_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem71").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem71(&mut self, mem71 : i64){
			root::set_int(self.ptr, "mem71", Qv::Val(mem71));
		}
		pub fn mem15(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem15").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem15_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem15").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem15(&mut self, mem15 : i64){
			root::set_int(self.ptr, "mem15", Qv::Val(mem15));
		}
		pub fn mem38(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem38").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem38_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem38").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem38(&mut self, mem38 : i64){
			root::set_int(self.ptr, "mem38", Qv::Val(mem38));
		}
		pub fn mem112(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem112").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem112_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem112").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem112(&mut self, mem112 : i64){
			root::set_int(self.ptr, "mem112", Qv::Val(mem112));
		}
		pub fn mem103(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem103").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem103_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem103").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem103(&mut self, mem103 : i64){
			root::set_int(self.ptr, "mem103", Qv::Val(mem103));
		}
		pub fn mem45(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem45").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem45_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem45").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem45(&mut self, mem45 : i64){
			root::set_int(self.ptr, "mem45", Qv::Val(mem45));
		}
		pub fn mem51(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem51").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem51_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem51").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem51(&mut self, mem51 : i64){
			root::set_int(self.ptr, "mem51", Qv::Val(mem51));
		}
		pub fn mem13(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem13").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem13_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem13").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem13(&mut self, mem13 : i64){
			root::set_int(self.ptr, "mem13", Qv::Val(mem13));
		}
		pub fn mem118(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem118").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem118_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem118").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem118(&mut self, mem118 : i64){
			root::set_int(self.ptr, "mem118", Qv::Val(mem118));
		}
		pub fn mem81(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem81").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem81_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem81").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem81(&mut self, mem81 : i64){
			root::set_int(self.ptr, "mem81", Qv::Val(mem81));
		}
		pub fn mem94(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem94").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem94_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem94").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem94(&mut self, mem94 : i64){
			root::set_int(self.ptr, "mem94", Qv::Val(mem94));
		}
		pub fn mem57(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem57").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem57_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem57").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem57(&mut self, mem57 : i64){
			root::set_int(self.ptr, "mem57", Qv::Val(mem57));
		}
		pub fn mem20(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem20").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem20_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem20").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem20(&mut self, mem20 : i64){
			root::set_int(self.ptr, "mem20", Qv::Val(mem20));
		}
		pub fn mem35(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem35").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem35_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem35").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem35(&mut self, mem35 : i64){
			root::set_int(self.ptr, "mem35", Qv::Val(mem35));
		}
		pub fn mem7(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem7").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem7_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem7").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem7(&mut self, mem7 : i64){
			root::set_int(self.ptr, "mem7", Qv::Val(mem7));
		}
		pub fn mem124(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem124").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem124_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem124").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem124(&mut self, mem124 : i64){
			root::set_int(self.ptr, "mem124", Qv::Val(mem124));
		}
		pub fn mem92(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem92").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem92_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem92").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem92(&mut self, mem92 : i64){
			root::set_int(self.ptr, "mem92", Qv::Val(mem92));
		}
		pub fn mem78(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem78").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem78_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem78").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem78(&mut self, mem78 : i64){
			root::set_int(self.ptr, "mem78", Qv::Val(mem78));
		}
		pub fn mem1(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem1").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem1_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem1").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem1(&mut self, mem1 : i64){
			root::set_int(self.ptr, "mem1", Qv::Val(mem1));
		}
		pub fn mem122(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem122").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem122_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem122").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem122(&mut self, mem122 : i64){
			root::set_int(self.ptr, "mem122", Qv::Val(mem122));
		}
		pub fn mem98(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem98").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem98_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem98").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem98(&mut self, mem98 : i64){
			root::set_int(self.ptr, "mem98", Qv::Val(mem98));
		}
		pub fn mem63(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem63").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem63_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem63").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem63(&mut self, mem63 : i64){
			root::set_int(self.ptr, "mem63", Qv::Val(mem63));
		}
		pub fn mem72(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem72").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem72_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem72").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem72(&mut self, mem72 : i64){
			root::set_int(self.ptr, "mem72", Qv::Val(mem72));
		}
		pub fn mem39(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem39").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem39_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem39").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem39(&mut self, mem39 : i64){
			root::set_int(self.ptr, "mem39", Qv::Val(mem39));
		}
		pub fn mem111(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem111").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem111_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem111").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem111(&mut self, mem111 : i64){
			root::set_int(self.ptr, "mem111", Qv::Val(mem111));
		}
		pub fn mem104(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem104").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem104_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem104").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem104(&mut self, mem104 : i64){
			root::set_int(self.ptr, "mem104", Qv::Val(mem104));
		}
		pub fn mem44(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem44").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem44_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem44").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem44(&mut self, mem44 : i64){
			root::set_int(self.ptr, "mem44", Qv::Val(mem44));
		}
		pub fn mem74(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem74").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem74_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem74").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem74(&mut self, mem74 : i64){
			root::set_int(self.ptr, "mem74", Qv::Val(mem74));
		}
		pub fn mem10(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem10").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem10_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem10").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem10(&mut self, mem10 : i64){
			root::set_int(self.ptr, "mem10", Qv::Val(mem10));
		}
		pub fn mem80(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem80").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem80_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem80").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem80(&mut self, mem80 : i64){
			root::set_int(self.ptr, "mem80", Qv::Val(mem80));
		}
		pub fn mem95(&self) -> i64{
			let qv = root::get_int(self.ptr, "mem95").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem95_def_val(&self) -> i64{
			let qv = root::get_int_def(self.ptr, "mem95").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem95(&mut self, mem95 : i64){
			root::set_int(self.ptr, "mem95", Qv::Val(mem95));
		}
	}
	
}
