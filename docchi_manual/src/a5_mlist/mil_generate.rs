use anyhow::Result;
use docchi::intf::generate_accessor_from_json_dir;

#[test]
fn mil_generate() -> Result<()> {
    generate_accessor_from_json_dir("src/a5_mlist/mil", "src/a5_mlist/mil_accessor.rs",true)?;
    Ok(())
}