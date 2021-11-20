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
	
		pub fn list1(&self) -> MListConst<List1MItem>{
			let mil = root::get_mlist_const(self.ptr, "list1").unwrap().unwrap();
			MListConst::new(mil, self)
		}
		pub fn list1_mut(&mut self) -> MListMut<List1MItem>{
			let mil = root::get_mlist_mut(self.ptr, "list1").unwrap();
			MListMut::new(mil, self)
		}
		pub fn list2(&self) -> MListConst<List2MItem>{
			let mil = root::get_mlist_const(self.ptr, "list2").unwrap().unwrap();
			MListConst::new(mil, self)
		}
		pub fn list2_mut(&mut self) -> MListMut<List2MItem>{
			let mil = root::get_mlist_mut(self.ptr, "list2").unwrap();
			MListMut::new(mil, self)
		}
		pub fn list3(&self) -> MListConst<List3MItem>{
			let mil = root::get_mlist_const(self.ptr, "list3").unwrap().unwrap();
			MListConst::new(mil, self)
		}
		pub fn list3_mut(&mut self) -> MListMut<List3MItem>{
			let mil = root::get_mlist_mut(self.ptr, "list3").unwrap();
			MListMut::new(mil, self)
		}
		pub fn refed1(&self) -> CTableConst<Refed1Table>{
			let t = Refed1Table::new(root::get_table(self.ptr.def(), "refed1").unwrap());
			CTableConst::new(t, self)
		}
		pub fn refed2(&self) -> CTableConst<Refed2Table>{
			let t = Refed2Table::new(root::get_table(self.ptr.def(), "refed2").unwrap());
			CTableConst::new(t, self)
		}
		pub fn refed3(&self) -> CTableConst<Refed3Table>{
			let t = Refed3Table::new(root::get_table(self.ptr.def(), "refed3").unwrap());
			CTableConst::new(t, self)
		}
		pub fn refed4(&self) -> CTableConst<Refed4Table>{
			let t = Refed4Table::new(root::get_table(self.ptr.def(), "refed4").unwrap());
			CTableConst::new(t, self)
		}
	}
	#[derive(Debug, PartialEq, Clone, Copy)]
	pub struct List1MItem {
		ptr : MItemPtr,
	}
	impl From<MItemPtr> for List1MItem {
		fn from(ptr : MItemPtr) -> Self {
			Self{ ptr }
		}
	}
	impl List1MItem {
		pub fn ref_refed1(&self) -> Refed1CItem{
			let qv = mitem::get_ref(self.ptr, "refed1").unwrap();
			Refed1CItem::from(qv.into_value().unwrap())
		}
		pub fn ref_id_refed1(&self) -> &String{
			let qv = mitem::get_ref_id(self.ptr, "refed1").unwrap();
			qv.into_value().unwrap()
		}
		pub fn ref_refed2(&self) -> NullOr<Refed2CItem>{
			let qv = mitem::get_ref(self.ptr, "refed2").unwrap();
			NullOr::from_qv(qv).unwrap().map(|p| Refed2CItem::from(p))
		}
		pub fn ref_id_refed2(&self) -> NullOr<&String>{
			let qv = mitem::get_ref_id(self.ptr, "refed2").unwrap();
			NullOr::from_qv(qv).unwrap()
		}
		pub fn ref_refed3(&self) -> UndefOr<Refed3CItem>{
			let qv = mitem::get_ref(self.ptr, "refed3").unwrap();
			UndefOr::from_qv(qv).unwrap().map(|p| Refed3CItem::from(p))
		}
		pub fn ref_id_refed3(&self) -> UndefOr<&String>{
			let qv = mitem::get_ref_id(self.ptr, "refed3").unwrap();
			UndefOr::from_qv(qv).unwrap()
		}
		pub fn ref_refed4(&self) -> Qv<Refed4CItem>{
			let qv = mitem::get_ref(self.ptr, "refed4").unwrap();
			qv.map(|p| Refed4CItem::from(p))
		}
		pub fn ref_id_refed4(&self) -> Qv<&String>{
			let qv = mitem::get_ref_id(self.ptr, "refed4").unwrap();
			qv
		}
		pub fn set_ref_refed1(&mut self, id : Refed1TableID){
			mitem::set_ref(self.ptr, "refed1", Qv::Val(id.to_str().to_string()));
		}
		pub fn set_ref_refed2(&mut self, id : NullOr<Refed2TableID>){
			mitem::set_ref(self.ptr, "refed2", id.into_qv().map(|v| v.to_str().to_string()));
		}
		pub fn set_ref_refed3(&mut self, id : UndefOr<Refed3TableID>){
			mitem::set_ref(self.ptr, "refed3", id.into_qv().map(|v| v.to_str().to_string()));
		}
		pub fn set_ref_refed4(&mut self, id : Qv<Refed4TableID>){
			mitem::set_ref(self.ptr, "refed4", id.into_qv().map(|v| v.to_str().to_string()));
		}
	}
	
	#[derive(Debug, PartialEq, Clone, Copy)]
	pub struct List2MItem {
		ptr : MItemPtr,
	}
	impl From<MItemPtr> for List2MItem {
		fn from(ptr : MItemPtr) -> Self {
			Self{ ptr }
		}
	}
	impl List2MItem {
		pub fn ref_refed1(&self) -> Refed1CItem{
			let qv = mitem::get_ref(self.ptr, "refed1").unwrap();
			Refed1CItem::from(qv.into_value().unwrap())
		}
		pub fn ref_id_refed1(&self) -> &String{
			let qv = mitem::get_ref_id(self.ptr, "refed1").unwrap();
			qv.into_value().unwrap()
		}
		pub fn ref_refed2(&self) -> NullOr<Refed2CItem>{
			let qv = mitem::get_ref(self.ptr, "refed2").unwrap();
			NullOr::from_qv(qv).unwrap().map(|p| Refed2CItem::from(p))
		}
		pub fn ref_id_refed2(&self) -> NullOr<&String>{
			let qv = mitem::get_ref_id(self.ptr, "refed2").unwrap();
			NullOr::from_qv(qv).unwrap()
		}
		pub fn ref_refed3(&self) -> UndefOr<Refed3CItem>{
			let qv = mitem::get_ref(self.ptr, "refed3").unwrap();
			UndefOr::from_qv(qv).unwrap().map(|p| Refed3CItem::from(p))
		}
		pub fn ref_id_refed3(&self) -> UndefOr<&String>{
			let qv = mitem::get_ref_id(self.ptr, "refed3").unwrap();
			UndefOr::from_qv(qv).unwrap()
		}
		pub fn ref_refed4(&self) -> Qv<Refed4CItem>{
			let qv = mitem::get_ref(self.ptr, "refed4").unwrap();
			qv.map(|p| Refed4CItem::from(p))
		}
		pub fn ref_id_refed4(&self) -> Qv<&String>{
			let qv = mitem::get_ref_id(self.ptr, "refed4").unwrap();
			qv
		}
		pub fn set_ref_refed1(&mut self, id : Refed1TableID){
			mitem::set_ref(self.ptr, "refed1", Qv::Val(id.to_str().to_string()));
		}
		pub fn set_ref_refed2(&mut self, id : NullOr<Refed2TableID>){
			mitem::set_ref(self.ptr, "refed2", id.into_qv().map(|v| v.to_str().to_string()));
		}
		pub fn set_ref_refed3(&mut self, id : UndefOr<Refed3TableID>){
			mitem::set_ref(self.ptr, "refed3", id.into_qv().map(|v| v.to_str().to_string()));
		}
		pub fn set_ref_refed4(&mut self, id : Qv<Refed4TableID>){
			mitem::set_ref(self.ptr, "refed4", id.into_qv().map(|v| v.to_str().to_string()));
		}
	}
	
	#[derive(Debug, PartialEq, Clone, Copy)]
	pub struct List3MItem {
		ptr : MItemPtr,
	}
	impl From<MItemPtr> for List3MItem {
		fn from(ptr : MItemPtr) -> Self {
			Self{ ptr }
		}
	}
	impl List3MItem {
		pub fn ref_refed1(&self) -> Refed1CItem{
			let qv = mitem::get_ref(self.ptr, "refed1").unwrap();
			Refed1CItem::from(qv.into_value().unwrap())
		}
		pub fn ref_id_refed1(&self) -> &String{
			let qv = mitem::get_ref_id(self.ptr, "refed1").unwrap();
			qv.into_value().unwrap()
		}
		pub fn ref_refed2(&self) -> NullOr<Refed2CItem>{
			let qv = mitem::get_ref(self.ptr, "refed2").unwrap();
			NullOr::from_qv(qv).unwrap().map(|p| Refed2CItem::from(p))
		}
		pub fn ref_id_refed2(&self) -> NullOr<&String>{
			let qv = mitem::get_ref_id(self.ptr, "refed2").unwrap();
			NullOr::from_qv(qv).unwrap()
		}
		pub fn ref_refed3(&self) -> UndefOr<Refed3CItem>{
			let qv = mitem::get_ref(self.ptr, "refed3").unwrap();
			UndefOr::from_qv(qv).unwrap().map(|p| Refed3CItem::from(p))
		}
		pub fn ref_id_refed3(&self) -> UndefOr<&String>{
			let qv = mitem::get_ref_id(self.ptr, "refed3").unwrap();
			UndefOr::from_qv(qv).unwrap()
		}
		pub fn ref_refed4(&self) -> Qv<Refed4CItem>{
			let qv = mitem::get_ref(self.ptr, "refed4").unwrap();
			qv.map(|p| Refed4CItem::from(p))
		}
		pub fn ref_id_refed4(&self) -> Qv<&String>{
			let qv = mitem::get_ref_id(self.ptr, "refed4").unwrap();
			qv
		}
		pub fn set_ref_refed1(&mut self, id : Refed1TableID){
			mitem::set_ref(self.ptr, "refed1", Qv::Val(id.to_str().to_string()));
		}
		pub fn set_ref_refed2(&mut self, id : NullOr<Refed2TableID>){
			mitem::set_ref(self.ptr, "refed2", id.into_qv().map(|v| v.to_str().to_string()));
		}
		pub fn set_ref_refed3(&mut self, id : UndefOr<Refed3TableID>){
			mitem::set_ref(self.ptr, "refed3", id.into_qv().map(|v| v.to_str().to_string()));
		}
		pub fn set_ref_refed4(&mut self, id : Qv<Refed4TableID>){
			mitem::set_ref(self.ptr, "refed4", id.into_qv().map(|v| v.to_str().to_string()));
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
	
	#[derive(Debug, PartialEq)]
	pub struct Refed3Table {
		ptr : TablePtr,
	}
	impl Refed3Table {
		pub fn new(ptr : TablePtr) -> Refed3Table{ Refed3Table{ ptr } } 
		pub unsafe fn c2_us(&self) -> Refed3CItem {
			let ptr = table::get_value(self.ptr, "c2").unwrap();
			Refed3CItem::from(ptr)
		}
		pub fn c2(&self) -> CItemConst<Refed3CItem> {
			let ptr = table::get_value(self.ptr, "c2").unwrap();
			CItemConst::new(Refed3CItem::from(ptr), self)
		}
		pub unsafe fn c1_us(&self) -> Refed3CItem {
			let ptr = table::get_value(self.ptr, "c1").unwrap();
			Refed3CItem::from(ptr)
		}
		pub fn c1(&self) -> CItemConst<Refed3CItem> {
			let ptr = table::get_value(self.ptr, "c1").unwrap();
			CItemConst::new(Refed3CItem::from(ptr), self)
		}
		pub unsafe fn get_by_id_us(&self, id : Refed3TableID) -> Refed3CItem{
			match id{
				Refed3TableID::C2 => self.c2_us(),
				Refed3TableID::C1 => self.c1_us(),
			}
		}
		pub fn get_by_id(&self, id : Refed3TableID) -> CItemConst<Refed3CItem>{
			CItemConst::new(unsafe{ self.get_by_id_us(id) }, self)
		}
	}
	#[repr(u64)] pub enum Refed3TableID{ C2, C1, }
	impl Refed3TableID{
		pub fn from_str(id : &str) -> Option<Self>{
			match id{
				"c2" => Some(Self::C2),
				"c1" => Some(Self::C1),
				_ =>{ None }
			}
		}
		pub fn from_num(id : usize) -> Self{
			match id{
				0 => Self::C2,
				1 => Self::C1,
				_ => panic!("invalid ID num {} Refed3TableID", id),
			}
		}
		pub fn len() -> usize{ 2 }
		pub fn to_num(&self) -> usize{
			match self{
				Refed3TableID::C2 => 0,
				Refed3TableID::C1 => 1,
			}
		}
		pub fn metadata() -> &'static [&'static str]{
			&["c2", "c1", ]
		}
		pub fn to_str(&self) -> &'static str{
			Self::metadata()[self.to_num()]
		}
	}
	#[derive(Debug, PartialEq, Clone, Copy)]
	pub struct Refed3CItem {
		ptr : CItemPtr,
	}
	impl From<CItemPtr> for Refed3CItem {
		fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
	}
	impl Refed3CItem {
		pub fn not_so_important(&self) -> i64{
			let qv = citem::get_int(self.ptr, "notSoImportant").unwrap();
			qv.into_value().unwrap()
		}
		pub fn not_so_important_def_val(&self) -> i64{
			let qv = citem::get_int_def(self.ptr, "notSoImportant").unwrap();
			qv.into_value().unwrap()
		}
		
	}
	
	#[derive(Debug, PartialEq)]
	pub struct Refed4Table {
		ptr : TablePtr,
	}
	impl Refed4Table {
		pub fn new(ptr : TablePtr) -> Refed4Table{ Refed4Table{ ptr } } 
		pub unsafe fn d2_us(&self) -> Refed4CItem {
			let ptr = table::get_value(self.ptr, "d2").unwrap();
			Refed4CItem::from(ptr)
		}
		pub fn d2(&self) -> CItemConst<Refed4CItem> {
			let ptr = table::get_value(self.ptr, "d2").unwrap();
			CItemConst::new(Refed4CItem::from(ptr), self)
		}
		pub unsafe fn d1_us(&self) -> Refed4CItem {
			let ptr = table::get_value(self.ptr, "d1").unwrap();
			Refed4CItem::from(ptr)
		}
		pub fn d1(&self) -> CItemConst<Refed4CItem> {
			let ptr = table::get_value(self.ptr, "d1").unwrap();
			CItemConst::new(Refed4CItem::from(ptr), self)
		}
		pub unsafe fn get_by_id_us(&self, id : Refed4TableID) -> Refed4CItem{
			match id{
				Refed4TableID::D2 => self.d2_us(),
				Refed4TableID::D1 => self.d1_us(),
			}
		}
		pub fn get_by_id(&self, id : Refed4TableID) -> CItemConst<Refed4CItem>{
			CItemConst::new(unsafe{ self.get_by_id_us(id) }, self)
		}
	}
	#[repr(u64)] pub enum Refed4TableID{ D2, D1, }
	impl Refed4TableID{
		pub fn from_str(id : &str) -> Option<Self>{
			match id{
				"d2" => Some(Self::D2),
				"d1" => Some(Self::D1),
				_ =>{ None }
			}
		}
		pub fn from_num(id : usize) -> Self{
			match id{
				0 => Self::D2,
				1 => Self::D1,
				_ => panic!("invalid ID num {} Refed4TableID", id),
			}
		}
		pub fn len() -> usize{ 2 }
		pub fn to_num(&self) -> usize{
			match self{
				Refed4TableID::D2 => 0,
				Refed4TableID::D1 => 1,
			}
		}
		pub fn metadata() -> &'static [&'static str]{
			&["d2", "d1", ]
		}
		pub fn to_str(&self) -> &'static str{
			Self::metadata()[self.to_num()]
		}
	}
	#[derive(Debug, PartialEq, Clone, Copy)]
	pub struct Refed4CItem {
		ptr : CItemPtr,
	}
	impl From<CItemPtr> for Refed4CItem {
		fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
	}
	impl Refed4CItem {
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
