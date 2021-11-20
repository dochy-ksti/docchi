use std::io::Write;
use crate::error::CoreResult;
use std::path::Path;
use docchi_archiver2::{ArchiveData, read_archive_data_from_directory};
use crate::JSON_ARC_OPT;

pub fn archive_src_dir<P : AsRef<Path>, W : Write>(src_dir : P, writer : &mut W) -> CoreResult<()>{
    let archive_data : ArchiveData<()> = read_archive_data_from_directory(
        src_dir,
        &*JSON_ARC_OPT,
        |_name, _dat| (),
    )?;
    Ok(docchi_archiver2::write_archive(&archive_data, writer)?)
}