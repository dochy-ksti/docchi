use std::path::Path;
use crate::error::FsResult;
use crate::imp::common::path::reserved_filename::is_reserved_filename;
use crate::imp::common::path::hash_dir_path::hash_dir_path;

///Dochy files can be safely removed.
pub fn remove_dochy_file<P : AsRef<Path>>(save_dir : P, hash : u128, filename : &str) -> FsResult<()>{
    if is_reserved_filename(filename) == false{
        let dir = hash_dir_path(save_dir, hash);
        let file_path = dir.join(filename);
        Ok(std::fs::remove_file(file_path)?)
    } else{
        Err(format!("{} couldn't be removed. {} is a system file.", filename, filename))?
    }
}