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

	pub fn list(&self) -> MListConst<ListMItem>{
		let mil = root::get_mlist_const(self.ptr, "list").unwrap().unwrap();
		MListConst::new(mil, self)
	}
	pub fn list_mut(&mut self) -> MListMut<ListMItem>{
		let mil = root::get_mlist_mut(self.ptr, "list").unwrap().unwrap();
		MListMut::new(mil, self)
	}
	pub fn table_a(&self) -> CTableConst<TableATable>{
		let t = TableATable::new(root::get_table(self.ptr.def(), "tableA").unwrap());
		CTableConst::new(t, self)
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
	pub fn bar(&self) -> i64{
		let qv = mitem::get_int(self.ptr, "bar").unwrap();
		qv.into_value().unwrap()
	}
	pub fn bar_def_val(&self) -> i64{
		let qv = mitem::get_int_def(self.ptr, "bar").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_bar(&mut self, bar : i64){
		mitem::set_int(self.ptr, "bar", Qv::Val(bar));
	}
	pub fn ref_table_a(&self) -> TableACItem{
		let qv = mitem::get_ref(self.ptr, "tableA").unwrap();
		TableACItem::from(qv.into_value().unwrap())
	}
	pub fn ref_id_table_a(&self) -> String{
		let qv = mitem::get_ref_id(self.ptr, "tableA").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_ref_table_a(&mut self, id : TableATableID){
		mitem::set_ref(self.ptr, "tableA", Qv::Val(id.to_str().to_string()));
	}
}

#[derive(Debug, PartialEq)]
pub struct TableATable {
	ptr : TablePtr,
}
impl TableATable {
	pub fn new(ptr : TablePtr) -> TableATable{ TableATable{ ptr } } 
	pub unsafe fn item2_us(&self) -> TableACItem {
		let ptr = table::get_value(self.ptr, "item2").unwrap();
		TableACItem::from(ptr)
	}
	pub fn item2(&self) -> CItemConst<TableACItem> {
		let ptr = table::get_value(self.ptr, "item2").unwrap();
		CItemConst::new(TableACItem::from(ptr), self)
	}
	pub unsafe fn item1_us(&self) -> TableACItem {
		let ptr = table::get_value(self.ptr, "item1").unwrap();
		TableACItem::from(ptr)
	}
	pub fn item1(&self) -> CItemConst<TableACItem> {
		let ptr = table::get_value(self.ptr, "item1").unwrap();
		CItemConst::new(TableACItem::from(ptr), self)
	}
	pub unsafe fn get_by_id_us(&self, id : TableATableID) -> TableACItem{
		match id{
			TableATableID::Item2 => self.item2_us(),
			TableATableID::Item1 => self.item1_us(),
		}
	}
	pub fn get_by_id(&self, id : TableATableID) -> CItemConst<TableACItem>{
		CItemConst::new(unsafe{ self.get_by_id_us(id) }, self)
	}
}
#[repr(u64)] pub enum TableATableID{ Item2, Item1, }
impl TableATableID{
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
			_ => panic!("invalid ID num {} TableATableID", id),
		}
	}
	pub fn len() -> usize{ 2 }
	pub fn to_num(&self) -> usize{
		match self{
			TableATableID::Item2 => 0,
			TableATableID::Item1 => 1,
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
pub struct TableACItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for TableACItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl TableACItem {
	pub fn foo(&self) -> i64{
		let qv = citem::get_int(self.ptr, "foo").unwrap();
		qv.into_value().unwrap()
	}
	pub fn foo_def_val(&self) -> i64{
		let qv = citem::get_int_def(self.ptr, "foo").unwrap();
		qv.into_value().unwrap()
	}
	
}

