use docchi::intf::generate_interface;
use docchi::core::structs::RootObject;
use anyhow::Result;
use docchi::core::json_dir_to_root;

pub fn generate_intf_src(json_dir_path : &str, src_file_path : &str) -> Result<RootObject> {
    let mut root = json_dir_to_root(json_dir_path,true)?;
    let source = generate_interface(&mut root);
    std::fs::write(src_file_path, &source.to_string_with_cfg_test())?;

    Ok(root)
}


