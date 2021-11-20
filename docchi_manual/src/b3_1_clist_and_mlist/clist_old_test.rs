use docchi::core::json_dir_to_root;
use docchi::error::DpResult;
use crate::b3_1_clist_and_mlist::clist_old_accessor::RootIntf;

#[test]
fn clilst_old_test() -> DpResult<()> {
    let old = json_dir_to_root("src/b3_1_clist_and_mlist/jsons/clist_old", true)?;

    let r = RootIntf::new(old);
    let list = r.list();
    let mut iter = list.iter();

    assert_eq!(iter.next().unwrap().foo(), 1);
    assert_eq!(iter.next().unwrap().foo(), 2);
    assert_eq!(iter.next().unwrap().foo(), 0);
    Ok(())
}