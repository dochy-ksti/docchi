

#[cfg(test)]
mod tests {
    use docchi::core::structs::{RootObject};
    use anyhow::Result;
    use crate::diff::generated_test_list::test::{RootIntf, Refed1TableID};
    use docchi::core::json_dir_to_root;

    fn apply(current : &RootObject, path : &str) -> Result<RootIntf>{
        let mut moto = json_dir_to_root(path, false)?;

        let diff = docchi_diff::get_diff(&moto, current)?;
        docchi_diff::apply_diff(&mut moto, &mut diff.as_slice())?;
        Ok(RootIntf::new(moto))
    }

    #[test]
    fn test_modify() -> Result<()>{
        let json_dir_path = "src/diff/diff_list/";
        let root_obj = json_dir_to_root(json_dir_path, false)?;

        let mut intf = RootIntf::new(root_obj);

        intf.list_mut().first_mut().unwrap().set_mem(2);

        let intf = apply(intf.root_obj_ref(), json_dir_path)?;

        assert_eq!(intf.list().first().unwrap().mem(), 2);

        Ok(())
    }

    #[test]
    fn test_add_add_remove() -> Result<()>{
        let json_dir_path = "src/diff/diff_list/";
        let root_obj = json_dir_to_root(json_dir_path, false)?;

        let mut intf = RootIntf::new(root_obj);

        let mut list = intf.list_mut();
        list.insert_last();

        let mut hoge = list.insert_last();
        hoge.set_ref_refed1(Refed1TableID::A2);
        intf.list_mut().remove_first();

        let intf = apply(intf.root_obj_ref(), json_dir_path)?;
        let list = intf.list();
        assert_eq!(list.len(), 2);
        assert_eq!(list.first_id().unwrap(), 1);
        assert_eq!(list.last_id().unwrap(), 2);
        assert_eq!(list.last().unwrap().ref_id_refed1(), "a2");

        Ok(())
    }

    #[test]
    fn test_remove_add_add_add_remove_user(){
        match test_remove_add_add_add_remove(){
            Ok(_)=>{},
            Err(e) => println!("{}", e),
        }
    }

    fn test_remove_add_add_add_remove() -> Result<()>{
        let json_dir_path = "src/diff/diff_list/";
        let root_obj = json_dir_to_root(json_dir_path, false)?;

        let mut intf = RootIntf::new(root_obj);

        let mut list = intf.list_mut();
        list.remove_first();
        list.insert_last();
        list.insert_first();
        list.insert_last();
        let id = list.last_id().unwrap();
        list.insert_last();
        list.remove(id);

        let mut iter = list.iter();
        let (id, _item) = iter.next().unwrap();
        assert_eq!(id, 2);
        let (id, _item) = iter.next().unwrap();
        assert_eq!(id, 1);
        let (id, _item) = iter.next().unwrap();
        assert_eq!(id, 4);

        let intf = apply(intf.root_obj_ref(), json_dir_path)?;
        let list = intf.list();
        assert_eq!(list.len(), 3);
        let mut iter = list.iter();
        let (id, _item) = iter.next().unwrap();
        assert_eq!(id, 2);
        let (id, _item) = iter.next().unwrap();
        assert_eq!(id, 1);
        let (id, _item) = iter.next().unwrap();
        assert_eq!(id, 4);

        Ok(())
    }


    #[test]
    fn test_in_add_add_add_add_remove_user(){
        match test_remove_add_add_add_remove(){
            Ok(_)=>{},
            Err(e) => println!("{}", e),
        }
    }

    fn test_in_add_add_add_add_remove() -> Result<()>{
        let json_dir_path = "src/diff/diff_list/";
        let root_obj = json_dir_to_root(json_dir_path, false)?;

        let mut intf = RootIntf::new(root_obj);

        let mut list = intf.list_mut();
        list.insert_last();

        let mut last = list.last_mut().unwrap();
        let mut l = last.in_list_mut();
        l.remove_first();

        let mut first = list.first_mut().unwrap();
        let mut f = first.in_list_mut();
        f.insert_first();
        f.insert_first();
        f.insert_last();
        f.insert_last();
        f.remove_last();
        let mut item = f.last_mut().unwrap();
        item.set_in_mem(4);

        let intf = apply(intf.root_obj_ref(), json_dir_path)?;
        let list = intf.list();
        assert_eq!(list.len(), 2);

        let first = list.first().unwrap();
        let in_list = first.in_list();
        let mut iter = in_list.iter();
        let (id, _item) = iter.next().unwrap();
        assert_eq!(id, 2);
        let (id, _item) = iter.next().unwrap();
        assert_eq!(id, 1);
        let (id, _item) = iter.next().unwrap();
        assert_eq!(id, 0);
        let (id, _item) = iter.next().unwrap();
        assert_eq!(id, 4);

        let last = list.last().unwrap();
        let list = last.in_list();

        assert_eq!(list.len(), 0);

        for _item in list.iter(){
            println!("nai ");
        }
        println!("ok");
        Ok(())
    }





}