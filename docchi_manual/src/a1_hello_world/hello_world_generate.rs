use anyhow::Result;
use docchi::core::structs::RootObject;
use docchi::core::json_dir_to_root;
use docchi::intf::generate_interface;

#[test]
fn hello_world_generate() -> Result<()> {
    let mut root_obj : RootObject = json_dir_to_root("src/a1_hello_world/src_dir", true)?;

    let ans = generate_interface(&mut root_obj);
    std::fs::write(
        "src/a1_hello_world/hello_world_accessor.rs",
        &ans.to_string(),
    ).unwrap();
    Ok(())
}