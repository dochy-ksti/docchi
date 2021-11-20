use docchi::core::{adjust_versions, json_dir_to_root};
use super::old_accessor as old_accessor;
use super::new_accessor as new_accessor;
use crate::b3_conversion::new_accessor_wrapper::NewWrapper;
use docchi::error::DpResult;

#[test]
fn conversion_test() -> DpResult<()>{
    let old = json_dir_to_root("src/b3_conversion/jsons/old", true)?;
    let new = json_dir_to_root("src/b3_conversion/jsons/new", true)?;
    let r = adjust_versions(new, old, true)?;
    let r = new_accessor::RootIntf::new(r);
    let r = NewWrapper::new(r);
    assert_eq!(r.new_value(), 100);

    let old = json_dir_to_root("src/sample_test/sample_code_json/version_aware/old", true)?;
    let mut old = old_accessor::RootIntf::new(old);
    old.set_old_value(2);

    let new = json_dir_to_root("src/sample_test/sample_code_json/version_aware/new", true)?;
    let r = adjust_versions(new, old.deconstruct(), true)?;
    let r = new_accessor::RootIntf::new(r);
    let r = NewWrapper::new(r);
    assert_eq!(r.new_value(), 20);
    Ok(())
}
