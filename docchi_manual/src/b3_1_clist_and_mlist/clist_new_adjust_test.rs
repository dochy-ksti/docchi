use docchi::core::{json_dir_to_root, adjust_versions};
use anyhow::Result;
use crate::b3_1_clist_and_mlist::clist_old_accessor::RootIntf;

#[test]
fn clilst_new_adjust_test() -> Result<()> {
    let old = json_dir_to_root("src/b3_1_clist_and_mlist/jsons/clist_old", true)?;
    let new = json_dir_to_root("src/b3_1_clist_and_mlist/jsons/clist_new", true)?;

    let r = adjust_versions(new, old, true)?;

    let r = RootIntf::new(r);
    let list = r.list();
    let mut iter = list.iter();

    assert_eq!(iter.next().unwrap().foo(), 3);
    assert_eq!(iter.next().unwrap().foo(), 4);
    assert_eq!(iter.next().unwrap().foo(), -1);
    assert_eq!(iter.next().unwrap().foo(), 5);
    Ok(())
}