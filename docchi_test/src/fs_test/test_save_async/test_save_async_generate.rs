use anyhow::Result;
use docchi::core::structs::RootObject;
use docchi::core::json_dir_to_root;
use docchi::intf::generate_interface;

#[test]
fn test_save_async_generate() -> Result<()> {
    let mut root_obj : RootObject = json_dir_to_root("src/fs_test/test_save_async/src_dir", true)?;

    let ans = generate_interface(&mut root_obj);
    std::fs::write(
        "src/fs_test/test_save_async/test_save_async_accessor.rs",
        &ans.to_string(),
    ).unwrap();
    Ok(())
}