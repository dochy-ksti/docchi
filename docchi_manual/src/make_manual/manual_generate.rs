
use docchi::intf::{generate_accessor_from_json_dir};
use anyhow::Result;

#[test]
fn generate_old() -> Result<()>{
    generate_accessor_from_json_dir("src/make_manual/manual", "src/make_manual/manual_accessor.rs",true)?;
    Ok(())
}


