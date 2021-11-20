

#[cfg(test)]
mod tests {

    use docchi::core::structs::{Qv, UndefOr};
    use crate::diff::generated_test_params::test::RootIntf;
    use docchi::core::json_dir_to_root;
    use docchi::error::DpResult;
    use docchi_core::structs::NullOr;


    #[test]
    fn test_diff2() -> DpResult<()>{
        let json_dir_path ="src/diff/diff_param/";
        let root_obj = json_dir_to_root(json_dir_path, true)?;

        let mut intf = RootIntf::new(root_obj);

        intf.set_hoge_int(1);
        intf.set_hoge_float(1.0);
        intf.set_hoge_string("山原や".to_string());
        intf.set_hoge_bool(false);
        intf.set_hoge_int_array(vec![]);
        intf.set_hoge_float_array(vec![1.2, 3.4, 5.6, 7.8]);
        intf.set_hoge_binary(vec![4,5,6]);
        intf.set_hoge_int_hatena(NullOr::Val(1));
        intf.set_hoge_int_hatena_to_null(NullOr::Null);
        intf.set_hoge_int_hatena_null(NullOr::Val(1));
        intf.set_hoge_undefined(UndefOr::Val(1));
        intf.set_hoge_undefined_to_undef(UndefOr::Undefined);
        intf.set_hoge_undefined_undef(UndefOr::Val(1));
        intf.set_hoge_undef_null_to_null(Qv::Null);
        intf.set_hoge_undef_null_to_undef(Qv::Undefined);
        intf.set_hoge_undef_null_null(Qv::Val(1));
        intf.set_hoge_undef_null_undef(Qv::Val(1));
        intf.set_hoge_undef_null_null_to_undef(Qv::Undefined);
        intf.set_hoge_undef_null_undef_to_null(Qv::Null);

        let mut moto = json_dir_to_root(json_dir_path,false)?;

        let diff = docchi_diff::get_diff(&moto, intf.root_obj_ref()).or_else(|e| Err(e.to_string()))?;
        docchi_diff::apply_diff(&mut moto, &mut diff.as_slice()).or_else(|e| Err(e.to_string()))?;
        let intf = RootIntf::new(moto);


        assert_eq!(intf.hoge_int(), 1);
        assert_eq!(intf.hoge_float(), 1.0);
        assert_eq!(intf.hoge_string(), "山原や");
        assert_eq!(intf.hoge_bool(), false);
        assert_eq!(intf.hoge_int_array(), &vec![]);
        assert_eq!(intf.hoge_float_array(), &vec![1.2,3.4,5.6,7.8]);
        assert_eq!(intf.hoge_binary(), &vec![4,5,6]);
        assert_eq!(intf.hoge_int_hatena(), NullOr::Val(1));
        assert_eq!(intf.hoge_int_hatena_to_null(), NullOr::Null);
        assert_eq!(intf.hoge_int_hatena_null(), NullOr::Val(1));
        assert_eq!(intf.hoge_undefined(), UndefOr::Val(1));
        assert_eq!(intf.hoge_undefined_to_undef(), UndefOr::Undefined);
        assert_eq!(intf.hoge_undefined_undef(), UndefOr::Val(1));
        assert_eq!(intf.hoge_undef_null_to_null(), Qv::Null);
        assert_eq!(intf.hoge_undef_null_to_undef(), Qv::Undefined);
        assert_eq!(intf.hoge_undef_null_null(), Qv::Val(1));
        assert_eq!(intf.hoge_undef_null_undef(), Qv::Val(1));
        assert_eq!(intf.hoge_undef_null_null_to_undef(), Qv::Undefined);
        assert_eq!(intf.hoge_undef_null_undef_to_null(), Qv::Null);
        assert_eq!(intf.hoge_int(), 1);
        assert_eq!(intf.hoge_int(), 1);
        Ok(())
    }

    //二段階diffする
    fn test_diff3() -> DpResult<()>{
        let json_dir_path = "src/diff/diff_param/";
        let root_obj = json_dir_to_root(json_dir_path,true)?;

        let mut intf = RootIntf::new(root_obj);

        intf.set_hoge_int(2);
        intf.set_hoge_float(2.0);
        intf.set_hoge_string("山原やん".to_string());
        intf.set_hoge_bool(true);
        intf.set_hoge_int_array(vec![100]);
        intf.set_hoge_float_array(vec![1.12, 3.14, 5.16, 7.18]);
        intf.set_hoge_int_hatena(NullOr::Val(2));
        intf.set_hoge_int_hatena_to_null(NullOr::Val(3));
        intf.set_hoge_int_hatena_null(NullOr::Val(2));
        intf.set_hoge_undefined(UndefOr::Val(2));
        intf.set_hoge_undefined_to_undef(UndefOr::Val(4));
        intf.set_hoge_undefined_undef(UndefOr::Val(2));
        intf.set_hoge_undef_null_to_null(Qv::Val(3));
        intf.set_hoge_undef_null_to_undef(Qv::Val(4));
        intf.set_hoge_undef_null_null(Qv::Val(2));
        intf.set_hoge_undef_null_undef(Qv::Val(2));
        intf.set_hoge_undef_null_null_to_undef(Qv::Val(4));
        intf.set_hoge_undef_null_undef_to_null(Qv::Val(3));

        let mut applied1 = json_dir_to_root(json_dir_path, false)?;

        let diff1 = docchi_diff::get_diff(&applied1, intf.root_obj_ref()).or_else(|e| Err(e.to_string()))?;
        docchi_diff::apply_diff(&mut applied1, &mut diff1.as_slice()).or_else(|e| Err(e.to_string()))?;

        intf.set_hoge_int(1);
        intf.set_hoge_float(1.0);
        intf.set_hoge_string("山原や".to_string());
        intf.set_hoge_bool(false);
        intf.set_hoge_int_array(vec![]);
        intf.set_hoge_float_array(vec![1.2, 3.4, 5.6, 7.8]);
        intf.set_hoge_int_hatena(NullOr::Val(1));
        intf.set_hoge_int_hatena_to_null(NullOr::Null);
        intf.set_hoge_int_hatena_null(NullOr::Val(1));
        intf.set_hoge_undefined(UndefOr::Val(1));
        intf.set_hoge_undefined_to_undef(UndefOr::Undefined);
        intf.set_hoge_undefined_undef(UndefOr::Val(1));
        intf.set_hoge_undef_null_to_null(Qv::Null);
        intf.set_hoge_undef_null_to_undef(Qv::Undefined);
        intf.set_hoge_undef_null_null(Qv::Val(1));
        intf.set_hoge_undef_null_undef(Qv::Val(1));
        intf.set_hoge_undef_null_null_to_undef(Qv::Undefined);
        intf.set_hoge_undef_null_undef_to_null(Qv::Null);

        let mut applied2 = applied1.clone();
        let diff2 = docchi_diff::get_diff(&applied1, intf.root_obj_ref()).or_else(|e| Err(e.to_string()))?;
        docchi_diff::apply_diff(&mut applied2, &mut diff2.as_slice()).or_else(|e| Err(e.to_string()))?;

        let intf = RootIntf::new(applied2);

        assert_eq!(intf.hoge_int(), 1);
        assert_eq!(intf.hoge_float(), 1.0);
        assert_eq!(intf.hoge_string(), "山原や");
        assert_eq!(intf.hoge_bool(), false);
        assert_eq!(intf.hoge_int_array(), &vec![]);
        assert_eq!(intf.hoge_float_array(), &vec![1.2,3.4,5.6,7.8]);
        assert_eq!(intf.hoge_int_hatena(), NullOr::Val(1));
        assert_eq!(intf.hoge_int_hatena_to_null(), NullOr::Null);
        assert_eq!(intf.hoge_int_hatena_null(), NullOr::Val(1));
        assert_eq!(intf.hoge_undefined(), UndefOr::Val(1));
        assert_eq!(intf.hoge_undefined_to_undef(), UndefOr::Undefined);
        assert_eq!(intf.hoge_undefined_undef(), UndefOr::Val(1));
        assert_eq!(intf.hoge_undef_null_to_null(), Qv::Null);
        assert_eq!(intf.hoge_undef_null_to_undef(), Qv::Undefined);
        assert_eq!(intf.hoge_undef_null_null(), Qv::Val(1));
        assert_eq!(intf.hoge_undef_null_undef(), Qv::Val(1));
        assert_eq!(intf.hoge_undef_null_null_to_undef(), Qv::Undefined);
        assert_eq!(intf.hoge_undef_null_undef_to_null(), Qv::Null);
        assert_eq!(intf.hoge_int(), 1);
        assert_eq!(intf.hoge_int(), 1);
        Ok(())
    }


}