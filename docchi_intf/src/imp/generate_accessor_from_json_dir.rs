use std::path::Path;
use crate::{IntfResult, generate_interface};
use docchi_core::json_dir_to_root;

pub fn generate_accessor_from_json_dir<P : AsRef<Path>>(json_dir : P, output : P, validation : bool) -> IntfResult<()> {
    let r = json_dir_to_root(json_dir.as_ref(), validation)?;
    let ans = generate_interface(&r);
    std::fs::write(
        output.as_ref(),
        &ans.to_string()
    )?;
    Ok(())
}