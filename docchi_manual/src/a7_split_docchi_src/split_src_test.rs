use docchi::core::{json_dir_to_root};
use anyhow::Result;

#[test]
fn split_src_test() -> Result<()> {
    let json = json_dir_to_root("src/a7_split_docchi_src/jsons/json", true)?;
    let splitted = json_dir_to_root("src/a7_split_docchi_src/jsons/splitted", true)?;

    assert_eq!(json.default(), splitted.default());
    Ok(())
}