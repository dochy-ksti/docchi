

#[cfg(test)]
mod tests {
    use crate::diff::generated_test_big_structure::test::RootIntf;
    use docchi::core::json_dir_to_root;
    use docchi::error::DpResult;

    #[test]
    fn test_big_structure(){
        match test_mem1(){
            Ok(()) =>{},
            Err(s) => println!("{}", s),
        }

        match test_mem63_65(){
            Ok(()) =>{},
            Err(s) => println!("{}", s),
        }

    }


    #[test]
    fn test_zero() -> DpResult<()>{
        let json_dir_path = "src/diff/diff_big_structure/";
        let root_obj = json_dir_to_root(json_dir_path,false)?;

        let intf = RootIntf::new(root_obj);

        let mut moto = json_dir_to_root(json_dir_path, false)?;

        //zero
        let diff = docchi_diff::get_diff(&moto, intf.root_obj_ref())?;
        docchi_diff::apply_diff(&mut moto, &mut diff.as_slice())?;

        let moto2 = json_dir_to_root(json_dir_path, false)?;
        assert!(moto.contents_eq(&moto2));

        Ok(())
    }

    fn test_mem1() -> DpResult<()>{
        let json_dir_path = "src/diff/diff_big_structure/";
        let root_obj = json_dir_to_root(json_dir_path,false)?;

        let mut intf = RootIntf::new(root_obj);
        assert_eq!(intf.mem1(), 2);
        intf.set_mem1(1);

        let mut moto = json_dir_to_root(json_dir_path, false)?;

        let diff = docchi_diff::get_diff(&moto, intf.root_obj_ref())?;
        docchi_diff::apply_diff(&mut moto,&mut  diff.as_slice())?;
        let intf = RootIntf::new(moto);

        assert_eq!(intf.mem1(), 1);
        Ok(())
    }

    fn test_mem63_65() -> DpResult<()>{
        let json_dir_path = "src/diff/diff_big_structure/";
        let root_obj = json_dir_to_root(json_dir_path, false)?;

        let mut intf = RootIntf::new(root_obj);
        intf.set_mem63(1);
        intf.set_mem64(1);
        intf.set_mem65(1);

        let mut moto = json_dir_to_root(json_dir_path,false)?;

        let diff = docchi_diff::get_diff(&moto, intf.root_obj_ref())?;
        docchi_diff::apply_diff(&mut moto, &mut diff.as_slice())?;
        let intf = RootIntf::new(moto);

        assert_eq!(intf.mem63(), 1);
        assert_eq!(intf.mem64(), 1);
        assert_eq!(intf.mem65(), 1);
        Ok(())
    }

    #[test]
    fn test_mem64_65() -> DpResult<()>{
        let json_dir_path = "src/diff/diff_big_structure/";
        let root_obj = json_dir_to_root(json_dir_path, false)?;

        let mut intf = RootIntf::new(root_obj);
        intf.set_mem64(1);
        intf.set_mem65(1);

        let mut moto = json_dir_to_root(json_dir_path, false)?;

        //store ids はnumbers
        // let diff = docchi_diff::get_kvals(&moto, unsafe{ intf.root_obj_ref() }).or_else(|e| Err(e.message))?;
        //println!("{:?}", diff);
        let diff = docchi_diff::get_diff(&moto, intf.root_obj_ref())?;
        docchi_diff::apply_diff(&mut moto, &mut diff.as_slice())?;
        let intf = RootIntf::new(moto);

        assert_eq!(intf.mem64(), 1);
        assert_eq!(intf.mem65(), 1);
        Ok(())
    }

    #[test]
    fn test_mem58_62() -> DpResult<()>{
        let json_dir_path = "src/diff/diff_big_structure/";
        let root_obj = json_dir_to_root(json_dir_path, false)?;

        let mut intf = RootIntf::new(root_obj);
        intf.set_mem58(1);
        intf.set_mem59(1);
        intf.set_mem60(1);
        intf.set_mem61(1);
        intf.set_mem62(1);

        let mut moto = json_dir_to_root(json_dir_path, false)?;

        //numbers
        let diff = docchi_diff::get_diff(&moto, intf.root_obj_ref())?;
        docchi_diff::apply_diff(&mut moto, &mut diff.as_slice())?;
        let intf = RootIntf::new(moto);
        assert_eq!(intf.mem58(), 1);
        assert_eq!(intf.mem59(), 1);
        assert_eq!(intf.mem60(), 1);
        assert_eq!(intf.mem61(), 1);
        assert_eq!(intf.mem62(), 1);

        Ok(())
    }

    #[test]
    fn test_mem58_63() -> DpResult<()>{
        let json_dir_path = "src/diff/diff_big_structure/";
        let root_obj = json_dir_to_root(json_dir_path, false)?;

        let mut intf = RootIntf::new(root_obj);
        intf.set_mem58(1);
        intf.set_mem59(1);
        intf.set_mem60(1);
        intf.set_mem61(1);
        intf.set_mem62(1);
        intf.set_mem63(1);
        //intf.set_mem64(1);

        let mut moto = json_dir_to_root(json_dir_path, false)?;

        //u64
        let diff = docchi_diff::get_diff(&moto, intf.root_obj_ref())?;
        docchi_diff::apply_diff(&mut moto, &mut diff.as_slice())?;
        let intf = RootIntf::new(moto);
        assert_eq!(intf.mem58(), 1);
        assert_eq!(intf.mem59(), 1);
        assert_eq!(intf.mem60(), 1);
        assert_eq!(intf.mem61(), 1);
        assert_eq!(intf.mem62(), 1);
        assert_eq!(intf.mem63(), 1);
        //assert_eq!(intf.mem64(), 1);

        Ok(())
    }





}