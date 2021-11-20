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
	
		pub fn refed2(&self) -> CTableConst<Refed2Table>{
			let t = Refed2Table::new(root::get_table(self.ptr.def(), "refed2").unwrap());
			CTableConst::new(t, self)
		}
		pub fn list(&self) -> MListConst<ListMItem>{
			let mil = root::get_mlist_const(self.ptr, "list").unwrap().unwrap();
			MListConst::new(mil, self)
		}
		pub fn list_mut(&mut self) -> MListMut<ListMItem>{
			let mil = root::get_mlist_mut(self.ptr, "list").unwrap();
			MListMut::new(mil, self)
		}
		pub fn refed1(&self) -> CTableConst<Refed1Table>{
			let t = Refed1Table::new(root::get_table(self.ptr.def(), "refed1").unwrap());
			CTableConst::new(t, self)
		}
	}
	#[derive(Debug, PartialEq)]
	pub struct Refed2Table {
		ptr : TablePtr,
	}
	impl Refed2Table {
		pub fn new(ptr : TablePtr) -> Refed2Table{ Refed2Table{ ptr } } 
		pub unsafe fn b1_us(&self) -> Refed2CItem {
			let ptr = table::get_value(self.ptr, "b1").unwrap();
			Refed2CItem::from(ptr)
		}
		pub fn b1(&self) -> CItemConst<Refed2CItem> {
			let ptr = table::get_value(self.ptr, "b1").unwrap();
			CItemConst::new(Refed2CItem::from(ptr), self)
		}
		pub unsafe fn b2_us(&self) -> Refed2CItem {
			let ptr = table::get_value(self.ptr, "b2").unwrap();
			Refed2CItem::from(ptr)
		}
		pub fn b2(&self) -> CItemConst<Refed2CItem> {
			let ptr = table::get_value(self.ptr, "b2").unwrap();
			CItemConst::new(Refed2CItem::from(ptr), self)
		}
		pub unsafe fn get_by_id_us(&self, id : Refed2TableID) -> Refed2CItem{
			match id{
				Refed2TableID::B1 => self.b1_us(),
				Refed2TableID::B2 => self.b2_us(),
			}
		}
		pub fn get_by_id(&self, id : Refed2TableID) -> CItemConst<Refed2CItem>{
			CItemConst::new(unsafe{ self.get_by_id_us(id) }, self)
		}
	}
	#[repr(u64)] pub enum Refed2TableID{ B1, B2, }
	impl Refed2TableID{
		pub fn from_str(id : &str) -> Option<Self>{
			match id{
				"b1" => Some(Self::B1),
				"b2" => Some(Self::B2),
				_ =>{ None }
			}
		}
		pub fn from_num(id : usize) -> Self{
			match id{
				0 => Self::B1,
				1 => Self::B2,
				_ => panic!("invalid ID num {} Refed2TableID", id),
			}
		}
		pub fn len() -> usize{ 2 }
		pub fn to_num(&self) -> usize{
			match self{
				Refed2TableID::B1 => 0,
				Refed2TableID::B2 => 1,
			}
		}
		pub fn metadata() -> &'static [&'static str]{
			&["b1", "b2", ]
		}
		pub fn to_str(&self) -> &'static str{
			Self::metadata()[self.to_num()]
		}
	}
	#[derive(Debug, PartialEq, Clone, Copy)]
	pub struct Refed2CItem {
		ptr : CItemPtr,
	}
	impl From<CItemPtr> for Refed2CItem {
		fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
	}
	impl Refed2CItem {
		pub fn not_very_important(&self) -> i64{
			let qv = citem::get_int(self.ptr, "notVeryImportant").unwrap();
			qv.into_value().unwrap()
		}
		pub fn not_very_important_def_val(&self) -> i64{
			let qv = citem::get_int_def(self.ptr, "notVeryImportant").unwrap();
			qv.into_value().unwrap()
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
		pub fn mem(&self) -> i64{
			let qv = mitem::get_int(self.ptr, "mem").unwrap();
			qv.into_value().unwrap()
		}
		pub fn mem_def_val(&self) -> i64{
			let qv = mitem::get_int_def(self.ptr, "mem").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_mem(&mut self, mem : i64){
			mitem::set_int(self.ptr, "mem", Qv::Val(mem));
		}
		pub fn in_list(&self) -> MListConst<InListMItem>{
			let mil = mitem::get_mil_const(self.ptr, "inList").unwrap().unwrap();
			MListConst::new(mil, self)
		}
		pub fn in_list_mut(&mut self) -> MListMut<InListMItem>{
			let p = mitem::get_mil_mut(self.ptr, "inList").unwrap();
			MListMut::new(p, self)
		}
		pub fn ref_refed1(&self) -> Refed1CItem{
			let qv = mitem::get_ref(self.ptr, "refed1").unwrap();
			Refed1CItem::from(qv.into_value().unwrap())
		}
		pub fn ref_id_refed1(&self) -> &String{
			let qv = mitem::get_ref_id(self.ptr, "refed1").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_ref_refed1(&mut self, id : Refed1TableID){
			mitem::set_ref(self.ptr, "refed1", Qv::Val(id.to_str().to_string()));
		}
	}
	#[derive(Debug, PartialEq, Clone, Copy)]
	pub struct InListMItem {
		ptr : MItemPtr,
	}
	impl From<MItemPtr> for InListMItem {
		fn from(ptr : MItemPtr) -> Self {
			Self{ ptr }
		}
	}
	impl InListMItem {
		pub fn in_mem(&self) -> i64{
			let qv = mitem::get_int(self.ptr, "inMem").unwrap();
			qv.into_value().unwrap()
		}
		pub fn in_mem_def_val(&self) -> i64{
			let qv = mitem::get_int_def(self.ptr, "inMem").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_in_mem(&mut self, in_mem : i64){
			mitem::set_int(self.ptr, "inMem", Qv::Val(in_mem));
		}
		pub fn in_list2(&self) -> MListConst<InList2MItem>{
			let mil = mitem::get_mil_const(self.ptr, "inList2").unwrap().unwrap();
			MListConst::new(mil, self)
		}
		pub fn in_list2_mut(&mut self) -> MListMut<InList2MItem>{
			let p = mitem::get_mil_mut(self.ptr, "inList2").unwrap();
			MListMut::new(p, self)
		}
		pub fn ref_refed2(&self) -> Refed2CItem{
			let qv = mitem::get_ref(self.ptr, "refed2").unwrap();
			Refed2CItem::from(qv.into_value().unwrap())
		}
		pub fn ref_id_refed2(&self) -> &String{
			let qv = mitem::get_ref_id(self.ptr, "refed2").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_ref_refed2(&mut self, id : Refed2TableID){
			mitem::set_ref(self.ptr, "refed2", Qv::Val(id.to_str().to_string()));
		}
	}
	#[derive(Debug, PartialEq, Clone, Copy)]
	pub struct InList2MItem {
		ptr : MItemPtr,
	}
	impl From<MItemPtr> for InList2MItem {
		fn from(ptr : MItemPtr) -> Self {
			Self{ ptr }
		}
	}
	impl InList2MItem {
		pub fn in_mem2(&self) -> i64{
			let qv = mitem::get_int(self.ptr, "inMem2").unwrap();
			qv.into_value().unwrap()
		}
		pub fn in_mem2_def_val(&self) -> i64{
			let qv = mitem::get_int_def(self.ptr, "inMem2").unwrap();
			qv.into_value().unwrap()
		}
		pub fn set_in_mem2(&mut self, in_mem2 : i64){
			mitem::set_int(self.ptr, "inMem2", Qv::Val(in_mem2));
		}
		
		
	}
	
	#[derive(Debug, PartialEq)]
	pub struct Refed1Table {
		ptr : TablePtr,
	}
	impl Refed1Table {
		pub fn new(ptr : TablePtr) -> Refed1Table{ Refed1Table{ ptr } } 
		pub unsafe fn a1_us(&self) -> Refed1CItem {
			let ptr = table::get_value(self.ptr, "a1").unwrap();
			Refed1CItem::from(ptr)
		}
		pub fn a1(&self) -> CItemConst<Refed1CItem> {
			let ptr = table::get_value(self.ptr, "a1").unwrap();
			CItemConst::new(Refed1CItem::from(ptr), self)
		}
		pub unsafe fn a2_us(&self) -> Refed1CItem {
			let ptr = table::get_value(self.ptr, "a2").unwrap();
			Refed1CItem::from(ptr)
		}
		pub fn a2(&self) -> CItemConst<Refed1CItem> {
			let ptr = table::get_value(self.ptr, "a2").unwrap();
			CItemConst::new(Refed1CItem::from(ptr), self)
		}
		pub unsafe fn get_by_id_us(&self, id : Refed1TableID) -> Refed1CItem{
			match id{
				Refed1TableID::A1 => self.a1_us(),
				Refed1TableID::A2 => self.a2_us(),
			}
		}
		pub fn get_by_id(&self, id : Refed1TableID) -> CItemConst<Refed1CItem>{
			CItemConst::new(unsafe{ self.get_by_id_us(id) }, self)
		}
	}
	#[repr(u64)] pub enum Refed1TableID{ A1, A2, }
	impl Refed1TableID{
		pub fn from_str(id : &str) -> Option<Self>{
			match id{
				"a1" => Some(Self::A1),
				"a2" => Some(Self::A2),
				_ =>{ None }
			}
		}
		pub fn from_num(id : usize) -> Self{
			match id{
				0 => Self::A1,
				1 => Self::A2,
				_ => panic!("invalid ID num {} Refed1TableID", id),
			}
		}
		pub fn len() -> usize{ 2 }
		pub fn to_num(&self) -> usize{
			match self{
				Refed1TableID::A1 => 0,
				Refed1TableID::A2 => 1,
			}
		}
		pub fn metadata() -> &'static [&'static str]{
			&["a1", "a2", ]
		}
		pub fn to_str(&self) -> &'static str{
			Self::metadata()[self.to_num()]
		}
	}
	#[derive(Debug, PartialEq, Clone, Copy)]
	pub struct Refed1CItem {
		ptr : CItemPtr,
	}
	impl From<CItemPtr> for Refed1CItem {
		fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
	}
	impl Refed1CItem {
		pub fn not_important(&self) -> i64{
			let qv = citem::get_int(self.ptr, "notImportant").unwrap();
			qv.into_value().unwrap()
		}
		pub fn not_important_def_val(&self) -> i64{
			let qv = citem::get_int_def(self.ptr, "notImportant").unwrap();
			qv.into_value().unwrap()
		}
		
	}
	
	
}
