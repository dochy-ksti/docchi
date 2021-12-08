use anyhow::Result;
use docchi::intf::generate_accessor_from_json_dir;

#[test]
fn save_history_files_generate() -> Result<()> {
    generate_accessor_from_json_dir("src/b2_save_history_files/src_dir", "src/b2_save_history_files/save_history_files_accessor.rs",true)?;
    Ok(())
}