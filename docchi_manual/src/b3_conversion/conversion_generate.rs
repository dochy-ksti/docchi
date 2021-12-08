
use docchi::intf::{generate_accessor_from_json_dir};
use anyhow::Result;

#[test]
fn generate_old() -> Result<()>{
    generate_accessor_from_json_dir("src/b3_conversion/jsons/old", "src/b3_conversion/old_accessor.rs",true)?;
    Ok(())
}
#[test]
fn generate_new() -> Result<()>{
    generate_accessor_from_json_dir("src/b3_conversion/jsons/new", "src/b3_conversion/new_accessor.rs",true)?;
    Ok(())
}
#[test]
fn generate_new2() -> Result<()>{
    generate_accessor_from_json_dir("src/b3_conversion/jsons/new2", "src/b3_conversion/new2_accessor.rs",true)?;
    Ok(())
}

