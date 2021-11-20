

#[cfg(test)]
mod tests {
    use docchi::core::structs::Qv;
    use crate::diff::generated_test_ref::test::{RootIntf, Refed1TableID, Refed2TableID, Refed3TableID, Refed4TableID};
    use docchi::error::DpResult;
    use docchi::core::json_dir_to_root;
    use docchi_core::structs::{NullOr, UndefOr};

    #[test]
    fn test_diff2() -> DpResult<()>{
        let json_dir_path = "src/diff/diff_ref/";
        let root_obj = json_dir_to_root(json_dir_path, false)?;

        let mut intf = RootIntf::new(root_obj);

        let mut list1 = intf.list1_mut();
        let mut f = list1.first_mut().unwrap();
        f.set_ref_refed1(Refed1TableID::A2);
        f.set_ref_refed2(NullOr::Val(Refed2TableID::B2));
        f.set_ref_refed3(UndefOr::Val(Refed3TableID::C2));
        f.set_ref_refed4(Qv::Val(Refed4TableID::D2));


        let mut moto = json_dir_to_root(json_dir_path, false)?;

        let diff = docchi_diff::get_diff(&moto, intf.root_obj_ref()).or_else(|e| Err(e.to_string()))?;
        docchi_diff::apply_diff(&mut moto, &mut diff.as_slice()).or_else(|e| Err(e.to_string()))?;
        let mut intf = RootIntf::new(moto);
        let list1 = intf.list1();
        let f = list1.first().unwrap();


        assert_eq!(f.ref_id_refed1(), "a2");
        assert_eq!(f.ref_id_refed2(), NullOr::Val(&"b2".to_string()));
        assert_eq!(f.ref_id_refed3(), UndefOr::Val(&"c2".to_string()));
        assert_eq!(f.ref_id_refed4(), Qv::Val(&"d2".to_string()));

        let mut list1 = intf.list1_mut();
        let mut f = list1.last_mut().unwrap();
        f.set_ref_refed1(Refed1TableID::A1);
        f.set_ref_refed2(NullOr::Val(Refed2TableID::B1));
        f.set_ref_refed3(UndefOr::Val(Refed3TableID::C1));
        f.set_ref_refed4(Qv::Val(Refed4TableID::D1));

        let mut moto = json_dir_to_root(json_dir_path, false)?;

        let diff = docchi_diff::get_diff(&moto, intf.root_obj_ref())?;
        docchi_diff::apply_diff(&mut moto, &mut diff.as_slice())?;
        let mut intf = RootIntf::new(moto);
        let list1 = intf.list1();
        let f = list1.last().unwrap();

        assert_eq!(f.ref_id_refed1(), "a1");
        assert_eq!(f.ref_id_refed2(), NullOr::Val(&"b1".to_string()));
        assert_eq!(f.ref_id_refed3(), UndefOr::Val(&"c1".to_string()));
        assert_eq!(f.ref_id_refed4(), Qv::Val(&"d1".to_string()));

        let mut list2 = intf.list2_mut();
        let mut f = list2.first_mut().unwrap();
        f.set_ref_refed1(Refed1TableID::A2);
        f.set_ref_refed2(NullOr::Val(Refed2TableID::B2));
        f.set_ref_refed3(UndefOr::Val(Refed3TableID::C2));
        f.set_ref_refed4(Qv::Val(Refed4TableID::D2));


        let mut moto = json_dir_to_root(json_dir_path, false)?;

        let diff = docchi_diff::get_diff(&moto, intf.root_obj_ref())?;
        docchi_diff::apply_diff(&mut moto, &mut diff.as_slice())?;
        let mut intf = RootIntf::new(moto);
        let list2 = intf.list2();
        let f = list2.first().unwrap();

        assert_eq!(f.ref_id_refed1(), "a2");
        assert_eq!(f.ref_id_refed2(), NullOr::Val(&"b2".to_string()));
        assert_eq!(f.ref_id_refed3(), UndefOr::Val(&"c2".to_string()));
        assert_eq!(f.ref_id_refed4(), Qv::Val(&"d2".to_string()));

        let mut list2 = intf.list2_mut();
        let mut f = list2.last_mut().unwrap();
        //f.set_ref_refed1(Refed1TableID::A1);
        f.set_ref_refed2(NullOr::Null);
        f.set_ref_refed3(UndefOr::Undefined);
        f.set_ref_refed4(Qv::Null);

        let mut moto = json_dir_to_root(json_dir_path, false)?;

        let diff = docchi_diff::get_diff(&moto, intf.root_obj_ref())?;
        docchi_diff::apply_diff(&mut moto, &mut diff.as_slice())?;
        let mut intf = RootIntf::new(moto);
        let list2 = intf.list2();
        let f = list2.last().unwrap();

        //assert_eq!(f.ref_id_refed1(), "a1".to_string());
        assert_eq!(f.ref_id_refed2(), NullOr::Null);
        assert_eq!(f.ref_id_refed3(), UndefOr::Undefined);
        assert_eq!(f.ref_id_refed4(), Qv::Null);

        let mut list3 = intf.list3_mut();
        let mut f = list3.first_mut().unwrap();
        //f.set_ref_refed1(Refed1TableID::A2);
        f.set_ref_refed2(NullOr::Null);
        f.set_ref_refed3(UndefOr::Undefined);
        f.set_ref_refed4(Qv::Undefined);


        let mut moto = json_dir_to_root(json_dir_path, false)?;

        let diff = docchi_diff::get_diff(&moto, intf.root_obj_ref())?;
        docchi_diff::apply_diff(&mut moto, &mut diff.as_slice())?;
        let mut intf = RootIntf::new(moto);
        let list3 = intf.list3();
        let f = list3.first().unwrap();

        //assert_eq!(f.ref_id_refed1(), "a2".to_string());
        assert_eq!(f.ref_id_refed2(), NullOr::Null);
        assert_eq!(f.ref_id_refed3(), UndefOr::Undefined);
        assert_eq!(f.ref_id_refed4(), Qv::Undefined);

        let mut list3 = intf.list3_mut();
        let mut f = list3.last_mut().unwrap();
        //f.set_ref_refed1(Refed1TableID::A1);
        f.set_ref_refed2(NullOr::Null);
        f.set_ref_refed3(UndefOr::Undefined);
        f.set_ref_refed4(Qv::Undefined);

        let mut moto = json_dir_to_root(json_dir_path, false)?;

        let diff = docchi_diff::get_diff(&moto, intf.root_obj_ref())?;
        docchi_diff::apply_diff(&mut moto, &mut diff.as_slice())?;
        let intf = RootIntf::new(moto);
        let list3 = intf.list3();
        let f = list3.last().unwrap();

        //assert_eq!(f.ref_id_refed1(), "a1".to_string());
        assert_eq!(f.ref_id_refed2(), NullOr::Null);
        assert_eq!(f.ref_id_refed3(), UndefOr::Undefined);
        assert_eq!(f.ref_id_refed4(), Qv::Undefined);
        Ok(())
    }


}