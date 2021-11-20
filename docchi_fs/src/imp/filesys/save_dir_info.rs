use crate::common::CurrentSrc;
use std::path::{PathBuf, Path};
use docchi_core::structs::RootObject;
use crate::imp::filesys::save_cache_map::{get_cache, cache_and_get_info};
use crate::error::FsResult;

#[derive(Debug, Clone)]
pub struct SaveDirInfo{
    save_dir : PathBuf,
    current_src : CurrentSrc,
    hash : u128,
    src_root : RootObject,
}

impl SaveDirInfo{
    pub(crate) fn new(save_dir : PathBuf,
                      current_src : CurrentSrc,
                      hash : u128,
                      src_root : RootObject) -> SaveDirInfo{
        SaveDirInfo{ save_dir, current_src, hash, src_root }
    }

    pub fn create<P : AsRef<Path>>(save_dir : P, current_src : CurrentSrc) -> FsResult<SaveDirInfo>{
        cache_and_get_info(save_dir.as_ref(), current_src)
    }

    pub fn save_dir(&self) -> &Path{ &self.save_dir }
    pub fn current_src(&self) -> &CurrentSrc{ &self.current_src }
    pub fn hash(&self) -> u128{ self.hash }
    pub fn clone_src_root(&self) -> RootObject{ self.src_root.clone() }
    pub fn queued_threads(&self) -> usize{
        get_cache(self.save_dir()).unwrap().queued()
    }
}