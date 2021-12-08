use anyhow::Result;
use docchi::intf::generate_accessor_from_json_dir;

#[test]
fn clilst_old_generate() -> Result<()>{
    generate_accessor_from_json_dir("src/b3_1_clist_and_mlist/jsons/clist_old", "src/b3_1_clist_and_mlist/clist_old_accessor.rs",true)?;
    Ok(())
}