use docchi::core::json_dir_to_root;
use crate::sample_test::sample_code::clist_old_accessor::RootIntf;
use anyhow::Result;

#[test]
fn clilst_old_test() -> Result<()> {
    let old = json_dir_to_root("src/sample_test/sample_code_json/clist_old", true)?;

    let r = RootIntf::new(old);
    let list = r.list();
    let mut iter = list.iter();

    assert_eq!(iter.next().unwrap().foo(), 1);
    assert_eq!(iter.next().unwrap().foo(), 2);
    assert_eq!(iter.next().unwrap().foo(), 0);
    Ok(())
}