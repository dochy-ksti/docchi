use anyhow::Result;
use docchi::core::structs::RootObject;
use docchi::core::json_dir_to_root;
use docchi::intf::generate_interface;

#[test]
fn hello_history_generate() -> Result<()> {
    let mut root_obj : RootObject = json_dir_to_root("src/fs_test/history_accum_test/src_dir", true)?;

    let ans = generate_interface(&mut root_obj);
    std::fs::write(
        "src/fs_test/history_accum_test/hello_history_accessor.rs",
        &ans.to_string(),
    ).unwrap();
    Ok(())
}