use std::path::{Path, PathBuf};
use crate::common::CurrentSrc;
use crate::error::FsResult;
use crate::imp::common::path::hash_dir_path::hash_dir_path;
use std::fs::File;
use crate::imp::common::path::reserved_filename::ARCHIVE_DEFAULT_NAME;
use crate::common::reserved_filename::CREATED_TIME_FILE_NAME;
use crate::imp::common::path::created_time_file::create_time_dat;
use std::time::SystemTime;

pub(crate) fn prepare_hash_dir(proj_dir: &Path, src : &CurrentSrc, hash : u128) -> FsResult<PathBuf>{
    let hash_dir = get_or_make_hash_dir_path(proj_dir, hash)?;
    let archive_path = hash_dir.join(ARCHIVE_DEFAULT_NAME);
    if archive_path.exists() == false{
        let mut file = File::create(&archive_path)?;
        src.create_archive(&mut file)?;
    }
    Ok(hash_dir)
}

fn get_or_make_hash_dir_path(proj_dir: &Path, hash : u128) -> FsResult<PathBuf> {
    let hash_dir_path = hash_dir_path(proj_dir, hash);
    if hash_dir_path.exists() {
        return Ok(hash_dir_path);
    }
    std::fs::create_dir(&hash_dir_path)?;
    let time_file_path = hash_dir_path.join(CREATED_TIME_FILE_NAME);
    let mut file = File::create(time_file_path)?;
    create_time_dat(SystemTime::now(), &mut file)?;
    Ok(hash_dir_path)
}
