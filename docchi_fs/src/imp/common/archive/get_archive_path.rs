use std::path::{Path, PathBuf};
use crate::common::reserved_filename::ARCHIVE_DEFAULT_NAME;
use crate::common::hash_dir_path;

pub(crate) fn get_archive_path<P : AsRef<Path>>(dir_path : P) -> PathBuf{
    dir_path.as_ref().join(ARCHIVE_DEFAULT_NAME)
}

pub(crate) fn get_archive_path2<P : AsRef<Path>>(history_dir : P, hash : u128) -> PathBuf{
    get_archive_path(hash_dir_path(history_dir, hash))
}