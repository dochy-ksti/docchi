use anyhow::Result;
use docchi::intf::generate_accessor_from_json_dir;

#[test]
fn mlilst_new_generate() -> Result<()>{
    generate_accessor_from_json_dir("src/b3_1_clist_and_mlist/jsons/mlist_new", "src/b3_1_clist_and_mlist/mlist_new_accessor.rs",true)?;
    Ok(())
}