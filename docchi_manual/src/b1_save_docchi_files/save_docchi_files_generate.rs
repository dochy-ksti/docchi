use docchi::error::DpResult;
use docchi::intf::generate_accessor_from_json_dir;

#[test]
fn save_docchi_files_generate() -> DpResult<()> {
    generate_accessor_from_json_dir("src/b1_save_docchi_files/src_dir","src/b1_save_docchi_files/save_docchi_files_accessor.rs",true)?;
    Ok(())
}