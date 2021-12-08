
use docchi::core::{json_dir_to_root, adjust_versions};
use crate::sample_test::sample_code::version_awareness_accessor::RootIntf;
use anyhow::Result;
use crate::sample_test::sample_code::version_awareness_accessor_wrapper::VeraAccessorWrapper;
use docchi::core::intf::RootObjectPtr;

#[test]
fn version_awareness_test() -> Result<()>{
    let old = json_dir_to_root("src/sample_test/sample_code_json/version_aware/old", true)?;
    let new = json_dir_to_root("src/sample_test/sample_code_json/version_aware/new", true)?;
    let r = adjust_versions(new, old, true)?;
    let r =RootIntf::new(r);
    let r = VeraAccessorWrapper::new(r);
    assert_eq!(r.new_value(), 100);

    let mut old = json_dir_to_root("src/sample_test/sample_code_json/version_aware/old", true)?;
    docchi_core::intf::root::set_int(RootObjectPtr::new(&mut old), "oldValue", docchi_core::structs::Qv::Val(2));
    let new = json_dir_to_root("src/sample_test/sample_code_json/version_aware/new", true)?;
    let r = adjust_versions(new, old, true)?;
    let r =RootIntf::new(r);
    let r = VeraAccessorWrapper::new(r);
    assert_eq!(r.new_value(), 20);
    Ok(())
}
