use docchi::core::json_dir_to_root;
use anyhow::Result;
use crate::b3_1_clist_and_mlist::mlist_old_accessor::RootIntf;

#[test]
fn mlist_old_test() -> Result<()> {
    let old = json_dir_to_root("src/b3_1_clist_and_mlist/jsons/mlist_old", true)?;

    let mut r = RootIntf::new(old);
    let mut list = r.mlist_mut();
    let mut iter = list.iter();

    if let Some((_id, item)) = iter.next(){
        assert_eq!(item.bar(), 1);
        assert_eq!(item.baz(), 2);
    }
    if let Some((_id, item)) = iter.next(){
        assert_eq!(item.bar(), 0); // default value
        assert_eq!(item.baz(), 3);
    }
    if let Some((_id, item)) = iter.next(){
        assert_eq!(item.bar(), 3);
        assert_eq!(item.baz(), 1); // default value
    }
    Ok(())
}