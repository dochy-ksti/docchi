use docchi::error::DpResult;
use docchi::intf::generate_accessor_from_json_dir;

#[test]
fn mlilst_old_generate() -> DpResult<()>{
    generate_accessor_from_json_dir("src/b3_1_clist_and_mlist/jsons/mlist_old","src/b3_1_clist_and_mlist/mlist_old_accessor.rs",true)?;
    Ok(())
}