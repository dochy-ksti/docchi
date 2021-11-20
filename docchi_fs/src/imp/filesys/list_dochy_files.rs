use std::path::Path;
use crate::imp::common::list::file_data::FileData;
use crate::imp::common::list::list_files::list_files;
use crate::error::FsResult;

pub fn list_dochy_files<P : AsRef<Path>>(save_dir : P) -> FsResult<Vec<FileData>>{
    list_files(save_dir)
}