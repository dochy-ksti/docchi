// use docchi::core::intf::*;
// use docchi::core::structs::*;
// unsafe impl Send for RootIntf {}
// unsafe impl Sync for RootIntf {}
// #[derive(Debug)]
// pub struct RootIntf{
//     root : Box<RootObject>,
//     ptr : RootObjectPtr,
// }
// impl RootIntf{
//     pub fn new(obj : RootObject) -> RootIntf{
// 		let mut root = Box::new(obj);
// 		let ptr = RootObjectPtr::new(root.as_mut());
// 		RootIntf { root, ptr }
// 	}
//     pub fn root_obj_ref(&self) -> &RootObject{ self.root.as_ref() }
//     pub fn root_obj_ref_mut(&mut self) -> &mut RootObject{ self.root.as_mut() }
//     pub fn deconstruct(self) -> RootObject{ *self.root }
//
// 	pub fn pugya_int(&self) -> UndefOr<i64>{
// 		let qv = root::get_int(self.ptr, "pugyaInt").unwrap();
// 		UndefOr::from_qv(qv).unwrap()
// 	}
// 	pub fn pugya_int_def_val(&self) -> UndefOr<i64>{
// 		let qv = root::get_int_def(self.ptr, "pugyaInt").unwrap();
// 		UndefOr::from_qv(qv).unwrap()
// 	}
// 	pub fn set_pugya_int(&mut self, pugya_int : UndefOr<i64>){
// 		root::set_int(self.ptr, "pugyaInt", pugya_int.into_qv());
// 	}
// 	pub fn hoge_bool(&self) -> bool{
// 		let qv = root::get_bool(self.ptr, "hogeBool").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn hoge_bool_def_val(&self) -> bool{
// 		let qv = root::get_bool_def(self.ptr, "hogeBool").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn set_hoge_bool(&mut self, hoge_bool : bool){
// 		root::set_bool(self.ptr, "hogeBool", Qv::Val(hoge_bool));
// 	}
// 	pub fn at_huga_number(&self) -> i64{
// 		let qv = root::get_int(self.ptr, "@HugaNumber").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn at_huga_number_def_val(&self) -> i64{
// 		let qv = root::get_int_def(self.ptr, "@HugaNumber").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn set_at_huga_number(&mut self, at_huga_number : i64){
// 		root::set_int(self.ptr, "@HugaNumber", Qv::Val(at_huga_number));
// 	}
// 	pub fn ini_item_list(&self) -> CListConst<IniItemListCItem>{
// 		CListConst::new(root::get_clist(self.ptr, "iniItemList").unwrap(), self)
// 	}
// 	pub fn int_array_def_val(&self) -> &Vec<i64>{
// 		let qv = root::get_int_array_def(self.ptr, "intArray").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn int_array(&self) -> &Vec<i64>{
// 		let qv = root::get_immutable_int_array(self.ptr, "intArray").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn int_array_mut(&mut self) -> &mut Vec<i64>{
// 		let qv = root::get_mutable_int_array(self.ptr, "intArray").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn set_int_array(&mut self, int_array : Vec<i64>){
// 		root::set_int_array(self.ptr, "intArray", Qv::Val(int_array));
// 	}
// 	pub fn some_str_def_val(&self) -> &String{
// 		let qv = root::get_str_def(self.ptr, "someStr").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn some_str(&self) -> &String{
// 		let qv = root::get_immutable_str(self.ptr, "someStr").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn some_str_mut(&mut self) -> &mut String{
// 		let qv = root::get_mutable_str(self.ptr, "someStr").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn set_some_str(&mut self, some_str : String){
// 		root::set_str(self.ptr, "someStr", Qv::Val(some_str));
// 	}
// 	pub fn hego_float(&self) -> NullOr<f64>{
// 		let qv = root::get_float(self.ptr, "hegoFloat").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub fn hego_float_def_val(&self) -> NullOr<f64>{
// 		let qv = root::get_float_def(self.ptr, "hegoFloat").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub fn set_hego_float(&mut self, hego_float : NullOr<f64>){
// 		root::set_float(self.ptr, "hegoFloat", hego_float.into_qv());
// 	}
// 	pub fn new_name_def_val(&self) -> UndefOr<&String>{
// 		let qv = root::get_str_def(self.ptr, "newName").unwrap();
// 		UndefOr::from_qv(qv).unwrap()
// 	}
// 	pub fn new_name(&self) -> UndefOr<&String>{
// 		let qv = root::get_immutable_str(self.ptr, "newName").unwrap();
// 		UndefOr::from_qv(qv).unwrap()
// 	}
// 	pub fn new_name_mut(&mut self) -> UndefOr<&mut String>{
// 		let qv = root::get_mutable_str(self.ptr, "newName").unwrap();
// 		UndefOr::from_qv(qv).unwrap()
// 	}
// 	pub fn set_new_name(&mut self, new_name : UndefOr<String>){
// 		root::set_str(self.ptr, "newName", new_name.into_qv());
// 	}
// 	pub fn new_name2(&self) -> UndefOr<i64>{
// 		let qv = root::get_int(self.ptr, "newName2").unwrap();
// 		UndefOr::from_qv(qv).unwrap()
// 	}
// 	pub fn new_name2_def_val(&self) -> UndefOr<i64>{
// 		let qv = root::get_int_def(self.ptr, "newName2").unwrap();
// 		UndefOr::from_qv(qv).unwrap()
// 	}
// 	pub fn set_new_name2(&mut self, new_name2 : UndefOr<i64>){
// 		root::set_int(self.ptr, "newName2", new_name2.into_qv());
// 	}
// 	pub fn hoge_float(&self) -> f64{
// 		let qv = root::get_float(self.ptr, "hogeFloat").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn hoge_float_def_val(&self) -> f64{
// 		let qv = root::get_float_def(self.ptr, "hogeFloat").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn set_hoge_float(&mut self, hoge_float : f64){
// 		root::set_float(self.ptr, "hogeFloat", Qv::Val(hoge_float));
// 	}
// 	pub fn binary_def_val(&self) -> &Vec<u8>{
// 		let qv = root::get_binary_def(self.ptr, "binary").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn binary(&self) -> &Vec<u8>{
// 		let qv = root::get_immutable_binary(self.ptr, "binary").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn binary_mut(&mut self) -> &mut Vec<u8>{
// 		let qv = root::get_mutable_binary(self.ptr, "binary").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn set_binary(&mut self, binary : Vec<u8>){
// 		root::set_binary(self.ptr, "binary", Qv::Val(binary));
// 	}
// 	pub fn weapon(&self) -> CTableConst<WeaponTable>{
// 		let t = WeaponTable::new(root::get_table(self.ptr.def(), "weapon").unwrap());
// 		CTableConst::new(t, self)
// 	}
// 	pub fn float_array_def_val(&self) -> &Vec<f64>{
// 		let qv = root::get_float_array_def(self.ptr, "floatArray").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn float_array(&self) -> &Vec<f64>{
// 		let qv = root::get_immutable_float_array(self.ptr, "floatArray").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn float_array_mut(&mut self) -> &mut Vec<f64>{
// 		let qv = root::get_mutable_float_array(self.ptr, "floatArray").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn set_float_array(&mut self, float_array : Vec<f64>){
// 		root::set_float_array(self.ptr, "floatArray", Qv::Val(float_array));
// 	}
// 	pub fn nullable_int_array_def_val(&self) -> NullOr<&Vec<i64>>{
// 		let qv = root::get_int_array_def(self.ptr, "nullableIntArray").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub fn nullable_int_array(&self) -> NullOr<&Vec<i64>>{
// 		let qv = root::get_immutable_int_array(self.ptr, "nullableIntArray").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub fn nullable_int_array_mut(&mut self) -> NullOr<&mut Vec<i64>>{
// 		let qv = root::get_mutable_int_array(self.ptr, "nullableIntArray").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub fn set_nullable_int_array(&mut self, nullable_int_array : NullOr<Vec<i64>>){
// 		root::set_int_array(self.ptr, "nullableIntArray", nullable_int_array.into_qv());
// 	}
// 	pub fn hoge_list(&self) -> CTableConst<HogeListTable>{
// 		let t = HogeListTable::new(root::get_table(self.ptr.def(), "hogeList").unwrap());
// 		CTableConst::new(t, self)
// 	}
// 	pub fn some_data(&self) -> CTableConst<SomeDataTable>{
// 		let t = SomeDataTable::new(root::get_table(self.ptr.def(), "someData").unwrap());
// 		CTableConst::new(t, self)
// 	}
// 	pub fn hoge_int(&self) -> i64{
// 		let qv = root::get_int(self.ptr, "hogeInt").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn hoge_int_def_val(&self) -> i64{
// 		let qv = root::get_int_def(self.ptr, "hogeInt").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn set_hoge_int(&mut self, hoge_int : i64){
// 		root::set_int(self.ptr, "hogeInt", Qv::Val(hoge_int));
// 	}
// 	pub fn usable(&self) -> CTableConst<UsableTable>{
// 		let t = UsableTable::new(root::get_table(self.ptr.def(), "usable").unwrap());
// 		CTableConst::new(t, self)
// 	}
// 	pub fn item_list3(&self) -> CListConst<ItemList3CItem>{
// 		CListConst::new(root::get_clist(self.ptr, "itemList3").unwrap(), self)
// 	}
// 	pub fn mut1(&self) -> MListConst<Mut1MItem>{
// 		let mil = root::get_mlist_const(self.ptr, "mut1").unwrap().unwrap();
// 		MListConst::new(mil, self)
// 	}
// 	pub fn mut1_mut(&mut self) -> MListMut<Mut1MItem>{
// 		let mil = root::get_mlist_mut(self.ptr, "mut1").unwrap();
// 		MListMut::new(mil, self)
// 	}
// 	pub fn hego_int(&self) -> NullOr<i64>{
// 		let qv = root::get_int(self.ptr, "hegoInt").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub fn hego_int_def_val(&self) -> NullOr<i64>{
// 		let qv = root::get_int_def(self.ptr, "hegoInt").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub fn set_hego_int(&mut self, hego_int : NullOr<i64>){
// 		root::set_int(self.ptr, "hegoInt", hego_int.into_qv());
// 	}
// 	pub fn huga_list(&self) -> CTableConst<HugaListTable>{
// 		let t = HugaListTable::new(root::get_table(self.ptr.def(), "hugaList").unwrap());
// 		CTableConst::new(t, self)
// 	}
// 	pub fn dim2_list(&self) -> CTableConst<Dim2ListTable>{
// 		let t = Dim2ListTable::new(root::get_table(self.ptr.def(), "dim2List").unwrap());
// 		CTableConst::new(t, self)
// 	}
// 	pub fn unko_list(&self) -> CTableConst<UnkoListTable>{
// 		let t = UnkoListTable::new(root::get_table(self.ptr.def(), "unkoList").unwrap());
// 		CTableConst::new(t, self)
// 	}
// 	pub fn hoge_string_def_val(&self) -> &String{
// 		let qv = root::get_str_def(self.ptr, "hogeString").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn hoge_string(&self) -> &String{
// 		let qv = root::get_immutable_str(self.ptr, "hogeString").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn hoge_string_mut(&mut self) -> &mut String{
// 		let qv = root::get_mutable_str(self.ptr, "hogeString").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn set_hoge_string(&mut self, hoge_string : String){
// 		root::set_str(self.ptr, "hogeString", Qv::Val(hoge_string));
// 	}
// 	pub fn hego_list(&self) -> CTableConst<HegoListTable>{
// 		let t = HegoListTable::new(root::get_table(self.ptr.def(), "hegoList").unwrap());
// 		CTableConst::new(t, self)
// 	}
// 	pub fn mut2(&self) -> MListConst<Mut2MItem>{
// 		let mil = root::get_mlist_const(self.ptr, "mut2").unwrap().unwrap();
// 		MListConst::new(mil, self)
// 	}
// 	pub fn mut2_mut(&mut self) -> MListMut<Mut2MItem>{
// 		let mil = root::get_mlist_mut(self.ptr, "mut2").unwrap();
// 		MListMut::new(mil, self)
// 	}
// 	pub fn old_name2_old(&self) -> NullOr<i64>{
// 		let qv = root::get_int(self.ptr, "oldName2").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub fn old_name2_old_def_val(&self) -> NullOr<i64>{
// 		let qv = root::get_int_def(self.ptr, "oldName2").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub fn set_old_name2_old(&mut self, old_name2 : NullOr<i64>){
// 		root::set_int(self.ptr, "oldName2", old_name2.into_qv());
// 	}
// 	pub fn pugya_int2(&self) -> Qv<i64>{
// 		let qv = root::get_int(self.ptr, "pugyaInt2").unwrap();
// 		qv
// 	}
// 	pub fn pugya_int2_def_val(&self) -> Qv<i64>{
// 		let qv = root::get_int_def(self.ptr, "pugyaInt2").unwrap();
// 		qv
// 	}
// 	pub fn set_pugya_int2(&mut self, pugya_int2 : Qv<i64>){
// 		root::set_int(self.ptr, "pugyaInt2", pugya_int2.into_qv());
// 	}
// 	pub fn enum_list(&self) -> CListConst<EnumListCItem>{
// 		CListConst::new(root::get_clist(self.ptr, "enumList").unwrap(), self)
// 	}
// 	pub fn empty_int_array_def_val(&self) -> &Vec<i64>{
// 		let qv = root::get_int_array_def(self.ptr, "emptyIntArray").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn empty_int_array(&self) -> &Vec<i64>{
// 		let qv = root::get_immutable_int_array(self.ptr, "emptyIntArray").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn empty_int_array_mut(&mut self) -> &mut Vec<i64>{
// 		let qv = root::get_mutable_int_array(self.ptr, "emptyIntArray").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn set_empty_int_array(&mut self, empty_int_array : Vec<i64>){
// 		root::set_int_array(self.ptr, "emptyIntArray", Qv::Val(empty_int_array));
// 	}
// 	pub fn old_name_old_def_val(&self) -> NullOr<&String>{
// 		let qv = root::get_str_def(self.ptr, "oldName").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub fn old_name_old(&self) -> NullOr<&String>{
// 		let qv = root::get_immutable_str(self.ptr, "oldName").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub fn old_name_old_mut(&mut self) -> NullOr<&mut String>{
// 		let qv = root::get_mutable_str(self.ptr, "oldName").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub fn set_old_name_old(&mut self, old_name : NullOr<String>){
// 		root::set_str(self.ptr, "oldName", old_name.into_qv());
// 	}
// }
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct IniItemListCItem {
// 	ptr : CItemPtr,
// }
// impl From<CItemPtr> for IniItemListCItem {
// 	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
// }
// impl IniItemListCItem {
// 	pub fn atk(&self) -> NullOr<i64>{
// 		let qv = citem::get_int(self.ptr, "atk").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub fn atk_def_val(&self) -> NullOr<i64>{
// 		let qv = citem::get_int_def(self.ptr, "atk").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub fn ref_weapon(&self) -> WeaponCItem{
// 		let qv = citem::get_ref(self.ptr, "weapon").unwrap();
// 		WeaponCItem::from(qv.into_value().unwrap())
// 	}
// 	pub fn ref_id_weapon(&self) -> &String{
// 		let qv = citem::get_ref_id(self.ptr, "weapon").unwrap();
// 		qv.into_value().unwrap()
// 	}
// }
//
// #[derive(Debug, PartialEq)]
// pub struct WeaponTable {
// 	ptr : TablePtr,
// }
// impl WeaponTable {
// 	pub fn new(ptr : TablePtr) -> WeaponTable{ WeaponTable{ ptr } }
// }
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct WeaponCItem {
// 	ptr : CItemPtr,
// }
// impl From<CItemPtr> for WeaponCItem {
// 	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
// }
// impl WeaponCItem {
// 	pub fn atk(&self) -> i64{
// 		let qv = citem::get_int(self.ptr, "atk").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn atk_def_val(&self) -> i64{
// 		let qv = citem::get_int_def(self.ptr, "atk").unwrap();
// 		qv.into_value().unwrap()
// 	}
//
// }
//
// #[derive(Debug, PartialEq)]
// pub struct HogeListTable {
// 	ptr : TablePtr,
// }
// impl HogeListTable {
// 	pub fn new(ptr : TablePtr) -> HogeListTable{ HogeListTable{ ptr } }
// }
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct HogeListCItem {
// 	ptr : CItemPtr,
// }
// impl From<CItemPtr> for HogeListCItem {
// 	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
// }
// impl HogeListCItem {
// 	pub fn mem_def_val(&self) -> &String{
// 		let qv = citem::get_str_def(self.ptr, "mem").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn mem(&self) -> &String{
// 		let qv = citem::get_immutable_str(self.ptr, "mem").unwrap();
// 		qv.into_value().unwrap()
// 	}
//
// }
//
// #[derive(Debug, PartialEq)]
// pub struct SomeDataTable {
// 	ptr : TablePtr,
// }
// impl SomeDataTable {
// 	pub fn new(ptr : TablePtr) -> SomeDataTable{ SomeDataTable{ ptr } }
// }
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct SomeDataCItem {
// 	ptr : CItemPtr,
// }
// impl From<CItemPtr> for SomeDataCItem {
// 	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
// }
// impl SomeDataCItem {
// 	pub fn n(&self) -> f64{
// 		let qv = citem::get_float(self.ptr, "n").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn n_def_val(&self) -> f64{
// 		let qv = citem::get_float_def(self.ptr, "n").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn s_def_val(&self) -> &String{
// 		let qv = citem::get_str_def(self.ptr, "s").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn s(&self) -> &String{
// 		let qv = citem::get_immutable_str(self.ptr, "s").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn ref_unko_list(&self) -> UnkoListCItem{
// 		let qv = citem::get_ref(self.ptr, "unkoList").unwrap();
// 		UnkoListCItem::from(qv.into_value().unwrap())
// 	}
// 	pub fn ref_id_unko_list(&self) -> &String{
// 		let qv = citem::get_ref_id(self.ptr, "unkoList").unwrap();
// 		qv.into_value().unwrap()
// 	}
// }
//
// #[derive(Debug, PartialEq)]
// pub struct UsableTable {
// 	ptr : TablePtr,
// }
// impl UsableTable {
// 	pub fn new(ptr : TablePtr) -> UsableTable{ UsableTable{ ptr } }
// }
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct UsableCItem {
// 	ptr : CItemPtr,
// }
// impl From<CItemPtr> for UsableCItem {
// 	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
// }
// impl UsableCItem {
// 	pub fn num(&self) -> i64{
// 		let qv = citem::get_int(self.ptr, "num").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn num_def_val(&self) -> i64{
// 		let qv = citem::get_int_def(self.ptr, "num").unwrap();
// 		qv.into_value().unwrap()
// 	}
//
// }
//
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct ItemList3CItem {
// 	ptr : CItemPtr,
// }
// impl From<CItemPtr> for ItemList3CItem {
// 	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
// }
// impl ItemList3CItem {
// 	pub fn mem_override_def_val(&self) -> NullOr<&String>{
// 		let qv = citem::get_str_def(self.ptr, "memOverride").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub fn mem_override(&self) -> NullOr<&String>{
// 		let qv = citem::get_immutable_str(self.ptr, "memOverride").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// 	pub fn ref_hoge_list(&self) -> HogeListCItem{
// 		let qv = citem::get_ref(self.ptr, "hogeList").unwrap();
// 		HogeListCItem::from(qv.into_value().unwrap())
// 	}
// 	pub fn ref_id_hoge_list(&self) -> &String{
// 		let qv = citem::get_ref_id(self.ptr, "hogeList").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn ref_hego_list(&self) -> HegoListCItem{
// 		let qv = citem::get_ref(self.ptr, "hegoList").unwrap();
// 		HegoListCItem::from(qv.into_value().unwrap())
// 	}
// 	pub fn ref_id_hego_list(&self) -> &String{
// 		let qv = citem::get_ref_id(self.ptr, "hegoList").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn ref_huga_list(&self) -> NullOr<HugaListCItem>{
// 		let qv = citem::get_ref(self.ptr, "hugaList").unwrap();
// 		NullOr::from_qv(qv).unwrap().map(|p| HugaListCItem::from(p))
// 	}
// 	pub fn ref_id_huga_list(&self) -> NullOr<&String>{
// 		let qv = citem::get_ref_id(self.ptr, "hugaList").unwrap();
// 		NullOr::from_qv(qv).unwrap()
// 	}
// }
//
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct Mut1MItem {
// 	ptr : MItemPtr,
// }
// impl From<MItemPtr> for Mut1MItem {
// 	fn from(ptr : MItemPtr) -> Self {
// 		Self{ ptr }
// 	}
// }
// impl Mut1MItem {
// 	pub fn inner_mut_list(&self) -> MListConst<InnerMutListMItem>{
// 		let mil = mitem::get_mil_const(self.ptr, "innerMutList").unwrap().unwrap();
// 		MListConst::new(mil, self)
// 	}
// 	pub fn inner_mut_list_mut(&mut self) -> MListMut<InnerMutListMItem>{
// 		let p = mitem::get_mil_mut(self.ptr, "innerMutList").unwrap();
// 		MListMut::new(p, self)
// 	}
// 	pub fn some_name_def_val(&self) -> &String{
// 		let qv = mitem::get_str_def(self.ptr, "someName").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn some_name(&self) -> &String{
// 		let qv = mitem::get_immutable_str(self.ptr, "someName").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn some_name_mut(&mut self) -> &mut String{
// 		let qv = mitem::get_mutable_str(self.ptr, "someName").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn set_some_name(&mut self, some_name : String){
// 		mitem::set_str(self.ptr, "someName", Qv::Val(some_name));
// 	}
//
//
// }
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct InnerMutListMItem {
// 	ptr : MItemPtr,
// }
// impl From<MItemPtr> for InnerMutListMItem {
// 	fn from(ptr : MItemPtr) -> Self {
// 		Self{ ptr }
// 	}
// }
// impl InnerMutListMItem {
// 	pub fn inner_list_mem(&self) -> i64{
// 		let qv = mitem::get_int(self.ptr, "innerListMem").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn inner_list_mem_def_val(&self) -> i64{
// 		let qv = mitem::get_int_def(self.ptr, "innerListMem").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn set_inner_list_mem(&mut self, inner_list_mem : i64){
// 		mitem::set_int(self.ptr, "innerListMem", Qv::Val(inner_list_mem));
// 	}
// 	pub fn ref_hego_list(&self) -> HegoListCItem{
// 		let qv = mitem::get_ref(self.ptr, "hegoList").unwrap();
// 		HegoListCItem::from(qv.into_value().unwrap())
// 	}
// 	pub fn ref_id_hego_list(&self) -> &String{
// 		let qv = mitem::get_ref_id(self.ptr, "hegoList").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn set_ref_hego_list(&mut self, id : HegoListTableID){
// 		mitem::set_ref(self.ptr, "hegoList", Qv::Val(id.to_str().to_string()));
// 	}
// }
//
// #[derive(Debug, PartialEq)]
// pub struct HugaListTable {
// 	ptr : TablePtr,
// }
// impl HugaListTable {
// 	pub fn new(ptr : TablePtr) -> HugaListTable{ HugaListTable{ ptr } }
// }
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct HugaListCItem {
// 	ptr : CItemPtr,
// }
// impl From<CItemPtr> for HugaListCItem {
// 	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
// }
// impl HugaListCItem {
//
// }
//
// #[derive(Debug, PartialEq)]
// pub struct Dim2ListTable {
// 	ptr : TablePtr,
// }
// impl Dim2ListTable {
// 	pub fn new(ptr : TablePtr) -> Dim2ListTable{ Dim2ListTable{ ptr } }
// }
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct Dim2ListCItem {
// 	ptr : CItemPtr,
// }
// impl From<CItemPtr> for Dim2ListCItem {
// 	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
// }
// impl Dim2ListCItem {
// 	pub fn data_mem(&self) -> i64{
// 		let qv = citem::get_int(self.ptr, "dataMem").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn data_mem_def_val(&self) -> i64{
// 		let qv = citem::get_int_def(self.ptr, "dataMem").unwrap();
// 		qv.into_value().unwrap()
// 	}
//
// }
//
// #[derive(Debug, PartialEq)]
// pub struct UnkoListTable {
// 	ptr : TablePtr,
// }
// impl UnkoListTable {
// 	pub fn new(ptr : TablePtr) -> UnkoListTable{ UnkoListTable{ ptr } }
// }
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct UnkoListCItem {
// 	ptr : CItemPtr,
// }
// impl From<CItemPtr> for UnkoListCItem {
// 	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
// }
// impl UnkoListCItem {
// 	pub fn hoge_int(&self) -> i64{
// 		let qv = citem::get_int(self.ptr, "hogeInt").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn hoge_int_def_val(&self) -> i64{
// 		let qv = citem::get_int_def(self.ptr, "hogeInt").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn hoge_string_def_val(&self) -> &String{
// 		let qv = citem::get_str_def(self.ptr, "hogeString").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn hoge_string(&self) -> &String{
// 		let qv = citem::get_immutable_str(self.ptr, "hogeString").unwrap();
// 		qv.into_value().unwrap()
// 	}
//
// }
//
// #[derive(Debug, PartialEq)]
// pub struct HegoListTable {
// 	ptr : TablePtr,
// }
// impl HegoListTable {
// 	pub fn new(ptr : TablePtr) -> HegoListTable{ HegoListTable{ ptr } }
// }
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct HegoListCItem {
// 	ptr : CItemPtr,
// }
// impl From<CItemPtr> for HegoListCItem {
// 	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
// }
// impl HegoListCItem {
// 	pub fn mem_def_val(&self) -> &String{
// 		let qv = citem::get_str_def(self.ptr, "mem").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn mem(&self) -> &String{
// 		let qv = citem::get_immutable_str(self.ptr, "mem").unwrap();
// 		qv.into_value().unwrap()
// 	}
//
// }
//
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct Mut2MItem {
// 	ptr : MItemPtr,
// }
// impl From<MItemPtr> for Mut2MItem {
// 	fn from(ptr : MItemPtr) -> Self {
// 		Self{ ptr }
// 	}
// }
// impl Mut2MItem {
// 	pub fn mem_def_val(&self) -> &String{
// 		let qv = mitem::get_str_def(self.ptr, "mem").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn mem(&self) -> &String{
// 		let qv = mitem::get_immutable_str(self.ptr, "mem").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn mem_mut(&mut self) -> &mut String{
// 		let qv = mitem::get_mutable_str(self.ptr, "mem").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn set_mem(&mut self, mem : String){
// 		mitem::set_str(self.ptr, "mem", Qv::Val(mem));
// 	}
// 	pub fn get_enum(&self) -> Mut2Enum{
// 		let (list_name, _) = mitem::get_enum(self.ptr).unwrap();
// 		let p = if let Qv::Val(p) = mitem::get_ref(self.ptr, &list_name).unwrap(){ p } else { unreachable!() };
// 		Mut2Enum::new(&list_name,p)
// 	}
// 	pub fn get_enum_ids(&self) -> (String,String){
// 		mitem::get_enum(self.ptr).unwrap()
// 	}
// 	pub fn set_enum(&mut self, kind : Mut2Kind){
// 		let (list_name, id) = kind.id();
// 		mitem::set_enum(self.ptr, list_name, id);
// 	}
// }
// pub enum Mut2Enum{ HogeList(HogeListCItem), HegoList(HegoListCItem), HugaList(HugaListCItem), }
// impl Mut2Enum{
// 	pub fn new(list_name : &str, ptr : CItemPtr) -> Mut2Enum{
// 		match list_name{
// 			"hogeList" => Mut2Enum::HogeList(HogeListCItem::from(ptr)),
// 			"hegoList" => Mut2Enum::HegoList(HegoListCItem::from(ptr)),
// 			"hugaList" => Mut2Enum::HugaList(HugaListCItem::from(ptr)),
// 			_ => panic!("Mut2Enum there's no enum type named {}", &list_name),
// 		}
// 	}
// }
// pub enum Mut2Kind{ HogeList(HogeListTableID), HegoList(HegoListTableID), HugaList(HugaListTableID), }
// impl Mut2Kind{
// 	pub fn id(&self) -> (&'static str, &'static str){
// 		match self{
// 			Mut2Kind::HogeList(v) => ("hogeList", v.to_str()),
// 			Mut2Kind::HegoList(v) => ("hegoList", v.to_str()),
// 			Mut2Kind::HugaList(v) => ("hugaList", v.to_str()),
// 		}
// 	}
// }
//
// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct EnumListCItem {
// 	ptr : CItemPtr,
// }
// impl From<CItemPtr> for EnumListCItem {
// 	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
// }
// impl EnumListCItem {
// 	pub fn mem_def_val(&self) -> &String{
// 		let qv = citem::get_str_def(self.ptr, "mem").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn mem(&self) -> &String{
// 		let qv = citem::get_immutable_str(self.ptr, "mem").unwrap();
// 		qv.into_value().unwrap()
// 	}
// 	pub fn get_enum(&self) -> EnumListEnum{
// 		let (list_name, _) = citem::get_enum(self.ptr).unwrap();
// 		let p = if let Qv::Val(p) = citem::get_ref(self.ptr, &list_name).unwrap(){ p } else { unreachable!() };
// 		EnumListEnum::new(&list_name,p)
// 	}
// 	pub fn get_enum_ids(&self) -> (String,String){
// 		citem::get_enum(self.ptr).unwrap()
// 	}
// }
// pub enum EnumListEnum{ HogeList(HogeListCItem), HegoList(HegoListCItem), HugaList(HugaListCItem), }
// impl EnumListEnum{
// 	pub fn new(list_name : &str, ptr : CItemPtr) -> EnumListEnum{
// 		match list_name{
// 			"hogeList" => EnumListEnum::HogeList(HogeListCItem::from(ptr)),
// 			"hegoList" => EnumListEnum::HegoList(HegoListCItem::from(ptr)),
// 			"hugaList" => EnumListEnum::HugaList(HugaListCItem::from(ptr)),
// 			_ => panic!("EnumListEnum there's no enum type named {}", &list_name),
// 		}
// 	}
// }
//
