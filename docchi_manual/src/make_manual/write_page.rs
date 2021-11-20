use std::path::Path;
use docchi::error::DpResult;
use crate::make_manual::get_content::{get_md_filename};

pub(crate) fn write_page<P : AsRef<Path>>(src : &str, page : &str, manual_dir : P) -> DpResult<()>{
    let path = manual_dir.as_ref().join(get_md_filename(src)?);
    std::fs::write(path, page)?;
    Ok(())
}

pub(crate) fn write_index_page<P : AsRef<Path>>(page : &str, manual_dir : P) -> DpResult<()>{
    let path = manual_dir.as_ref().join("index.md");
    std::fs::write(path, page)?;
    Ok(())
}