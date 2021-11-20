use std::time::SystemTime;
use std::fs::DirEntry;
use crate::error::FsResult;
use std::path::{Path, PathBuf};
use crate::imp::common::path::hash_dir_path::hash_dir_path;

/// file's metadata
/// We presume we can get modified time of diff files under any circumstances.
/// list_saved_files doesn't work if this presumption is not true.
#[derive(Debug)]
pub struct FileData{
    hash : u128,
    modified : SystemTime,
    len : u64,
    name : String,
}

impl FileData{
    ///creates FileData
    pub fn new(hash : u128, modified : SystemTime, name : String, len : u64) -> FileData{
        FileData{ hash, modified, name, len }
    }

    /// creates FileData from metadata
    pub fn from(hash : u128, entry : &DirEntry) -> FsResult<FileData>{
        let metadata = entry.metadata()?;
        let modified = metadata.modified()?;
        let name = entry.file_name().to_string_lossy().to_string();
        let size = metadata.len();
        Ok(FileData::new(hash, modified, name, size))
    }

    /// hash of the Dochy source file
    pub fn hash(&self) -> u128{ self.hash }

    /// modified time
    pub fn modified(&self) -> SystemTime{ self.modified }

    /// filename
    pub fn name(&self) -> &str{ &self.name }

    /// file's size
    pub fn len(&self) -> u64{ self.len }

    /// calculate the path. FileData doesn't contain save_dir so this function needs it
    pub fn calc_path<P : AsRef<Path>>(&self, save_dir: P) -> PathBuf{
        let save_dir = save_dir.as_ref();
        let dir = hash_dir_path(save_dir, self.hash);
        dir.join(self.name())
    }
}
