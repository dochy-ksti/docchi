use docchi::core::{json_dir_to_root, adjust_versions};
use anyhow::Result;
use crate::b3_1_clist_and_mlist::undefiable_list_accessor::RootIntf;

#[test]
fn undefiable_list_test() -> Result<()> {
    let old = json_dir_to_root("src/b3_1_clist_and_mlist/jsons/no_data", true)?;
    let new = json_dir_to_root("src/b3_1_clist_and_mlist/jsons/undefiable_list_separated", true)?;

    // We call "adjust_versions" manually here.
    // "load_docchi_file"/"load_history_file" automatically adjusts versions, so calling this manually isn't necessary.
    let r = adjust_versions(new, old, true)?;

    let mut r = RootIntf::new(r);
    //"list" is undefined, therefore it returns None.
    assert!(r.list().is_none());
    let a = r.list_mut();
    // When "list_mut" is called for the first time, an empty list is created and inserted, and the list is returned.
    assert_eq!(a.len(), 0);
    Ok(())
}