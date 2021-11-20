use docchi::error::DpResult;
use docchi::intf::generate_accessor_from_json_dir;

#[test]
fn table_generate() -> DpResult<()> {
    generate_accessor_from_json_dir("src/a6_table/table", "src/a6_table/table_accessor.rs", true)?;
    Ok(())
}