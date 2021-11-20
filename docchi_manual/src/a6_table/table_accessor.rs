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

	pub fn table2(&self) -> CTableConst<Table2Table>{
		let t = Table2Table::new(root::get_table(self.ptr.def(), "table2").unwrap());
		CTableConst::new(t, self)
	}
	pub fn mlist(&self) -> MListConst<MlistMItem>{
		let mil = root::get_mlist_const(self.ptr, "mlist").unwrap().unwrap();
		MListConst::new(mil, self)
	}
	pub fn mlist_mut(&mut self) -> MListMut<MlistMItem>{
		let mil = root::get_mlist_mut(self.ptr, "mlist").unwrap();
		MListMut::new(mil, self)
	}
	pub fn a_list(&self) -> CListConst<AListCItem>{
		CListConst::new(root::get_clist(self.ptr, "a_list").unwrap(), self)
	}
	pub fn table1(&self) -> CTableConst<Table1Table>{
		let t = Table1Table::new(root::get_table(self.ptr.def(), "table1").unwrap());
		CTableConst::new(t, self)
	}
}
#[derive(Debug, PartialEq)]
pub struct Table2Table {
	ptr : TablePtr,
}
impl Table2Table {
	pub fn new(ptr : TablePtr) -> Table2Table{ Table2Table{ ptr } } 
	pub unsafe fn a_us(&self) -> Table2CItem {
		let ptr = table::get_value(self.ptr, "a").unwrap();
		Table2CItem::from(ptr)
	}
	pub fn a(&self) -> CItemConst<Table2CItem> {
		let ptr = table::get_value(self.ptr, "a").unwrap();
		CItemConst::new(Table2CItem::from(ptr), self)
	}
	pub unsafe fn b_us(&self) -> Table2CItem {
		let ptr = table::get_value(self.ptr, "b").unwrap();
		Table2CItem::from(ptr)
	}
	pub fn b(&self) -> CItemConst<Table2CItem> {
		let ptr = table::get_value(self.ptr, "b").unwrap();
		CItemConst::new(Table2CItem::from(ptr), self)
	}
	pub unsafe fn get_by_id_us(&self, id : Table2TableID) -> Table2CItem{
		match id{
			Table2TableID::A => self.a_us(),
			Table2TableID::B => self.b_us(),
		}
	}
	pub fn get_by_id(&self, id : Table2TableID) -> CItemConst<Table2CItem>{
		CItemConst::new(unsafe{ self.get_by_id_us(id) }, self)
	}
}
#[repr(u64)] pub enum Table2TableID{ A, B, }
impl Table2TableID{
	pub fn from_str(id : &str) -> Option<Self>{
		match id{
			"a" => Some(Self::A),
			"b" => Some(Self::B),
			_ =>{ None }
		}
	}
	pub fn from_num(id : usize) -> Self{
		match id{
			0 => Self::A,
			1 => Self::B,
			_ => panic!("invalid ID num {} Table2TableID", id),
		}
	}
	pub fn len() -> usize{ 2 }
	pub fn to_num(&self) -> usize{
		match self{
			Table2TableID::A => 0,
			Table2TableID::B => 1,
		}
	}
	pub fn metadata() -> &'static [&'static str]{
		&["a", "b", ]
	}
	pub fn to_str(&self) -> &'static str{
		Self::metadata()[self.to_num()]
	}
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Table2CItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for Table2CItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl Table2CItem {
	pub fn ref_table1(&self) -> Table1CItem{
		let qv = citem::get_ref(self.ptr, "table1").unwrap();
		Table1CItem::from(qv.into_value().unwrap())
	}
	pub fn ref_id_table1(&self) -> &String{
		let qv = citem::get_ref_id(self.ptr, "table1").unwrap();
		qv.into_value().unwrap()
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
	pub fn ref_table2(&self) -> NullOr<Table2CItem>{
		let qv = mitem::get_ref(self.ptr, "table2").unwrap();
		NullOr::from_qv(qv).unwrap().map(|p| Table2CItem::from(p))
	}
	pub fn ref_id_table2(&self) -> NullOr<&String>{
		let qv = mitem::get_ref_id(self.ptr, "table2").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn ref_table1(&self) -> Table1CItem{
		let qv = mitem::get_ref(self.ptr, "table1").unwrap();
		Table1CItem::from(qv.into_value().unwrap())
	}
	pub fn ref_id_table1(&self) -> &String{
		let qv = mitem::get_ref_id(self.ptr, "table1").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_ref_table2(&mut self, id : NullOr<Table2TableID>){
		mitem::set_ref(self.ptr, "table2", id.into_qv().map(|v| v.to_str().to_string()));
	}
	pub fn set_ref_table1(&mut self, id : Table1TableID){
		mitem::set_ref(self.ptr, "table1", Qv::Val(id.to_str().to_string()));
	}
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AListCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for AListCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl AListCItem {
	pub fn name_def_val(&self) -> &String{
		let qv = citem::get_str_def(self.ptr, "name").unwrap();
		qv.into_value().unwrap()
	}
	pub fn name(&self) -> &String{
		let qv = citem::get_immutable_str(self.ptr, "name").unwrap();
		qv.into_value().unwrap()
	}
	pub fn ref_table1(&self) -> Table1CItem{
		let qv = citem::get_ref(self.ptr, "table1").unwrap();
		Table1CItem::from(qv.into_value().unwrap())
	}
	pub fn ref_id_table1(&self) -> &String{
		let qv = citem::get_ref_id(self.ptr, "table1").unwrap();
		qv.into_value().unwrap()
	}
}

#[derive(Debug, PartialEq)]
pub struct Table1Table {
	ptr : TablePtr,
}
impl Table1Table {
	pub fn new(ptr : TablePtr) -> Table1Table{ Table1Table{ ptr } } 
	pub unsafe fn item2_us(&self) -> Table1CItem {
		let ptr = table::get_value(self.ptr, "item2").unwrap();
		Table1CItem::from(ptr)
	}
	pub fn item2(&self) -> CItemConst<Table1CItem> {
		let ptr = table::get_value(self.ptr, "item2").unwrap();
		CItemConst::new(Table1CItem::from(ptr), self)
	}
	pub unsafe fn item1_us(&self) -> Table1CItem {
		let ptr = table::get_value(self.ptr, "item1").unwrap();
		Table1CItem::from(ptr)
	}
	pub fn item1(&self) -> CItemConst<Table1CItem> {
		let ptr = table::get_value(self.ptr, "item1").unwrap();
		CItemConst::new(Table1CItem::from(ptr), self)
	}
	pub unsafe fn get_by_id_us(&self, id : Table1TableID) -> Table1CItem{
		match id{
			Table1TableID::Item2 => self.item2_us(),
			Table1TableID::Item1 => self.item1_us(),
		}
	}
	pub fn get_by_id(&self, id : Table1TableID) -> CItemConst<Table1CItem>{
		CItemConst::new(unsafe{ self.get_by_id_us(id) }, self)
	}
}
#[repr(u64)] pub enum Table1TableID{ Item2, Item1, }
impl Table1TableID{
	pub fn from_str(id : &str) -> Option<Self>{
		match id{
			"item2" => Some(Self::Item2),
			"item1" => Some(Self::Item1),
			_ =>{ None }
		}
	}
	pub fn from_num(id : usize) -> Self{
		match id{
			0 => Self::Item2,
			1 => Self::Item1,
			_ => panic!("invalid ID num {} Table1TableID", id),
		}
	}
	pub fn len() -> usize{ 2 }
	pub fn to_num(&self) -> usize{
		match self{
			Table1TableID::Item2 => 0,
			Table1TableID::Item1 => 1,
		}
	}
	pub fn metadata() -> &'static [&'static str]{
		&["item2", "item1", ]
	}
	pub fn to_str(&self) -> &'static str{
		Self::metadata()[self.to_num()]
	}
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Table1CItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for Table1CItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl Table1CItem {
	pub fn val(&self) -> i64{
		let qv = citem::get_int(self.ptr, "val").unwrap();
		qv.into_value().unwrap()
	}
	pub fn val_def_val(&self) -> i64{
		let qv = citem::get_int_def(self.ptr, "val").unwrap();
		qv.into_value().unwrap()
	}
	
}

