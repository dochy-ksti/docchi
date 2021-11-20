use std::path::{Path, PathBuf};
use crate::imp::common::path::hash::hash_to_folder_name;
use crate::common::get_hash_times;
use crate::error::FsResult;

/// hash dir's path can be calculated with proj_dir(save_dir or history_dir)'s path and hash value
pub fn hash_dir_path<P : AsRef<Path>>(proj_dir: P, hash : u128) -> PathBuf{
    let name = hash_to_folder_name(hash);
    proj_dir.as_ref().join(&name)
}

/// Gets paths of hash directories
pub fn hash_dir_paths<P : AsRef<Path>>(proj_dir: P) -> FsResult<impl Iterator<Item=PathBuf>>{
    let proj_dir = proj_dir.as_ref();
    let hash_times = get_hash_times(proj_dir)?;
    let proj_dir = PathBuf::from(proj_dir);
    Ok(hash_times.into_iter().map(move |(hash, _time)| hash_dir_path(&proj_dir, hash)))
}