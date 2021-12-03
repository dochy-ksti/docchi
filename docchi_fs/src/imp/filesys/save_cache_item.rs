use crate::common::CurrentSrc;
use std::path::{Path};
use docchi_core::structs::RootObject;
use crate::imp::filesys::save_dir_info::SaveDirInfo;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub(crate) struct SaveCacheItem{
    info : SaveDirInfo,
    queued : AtomicUsize,
}

impl SaveCacheItem{
    pub(crate) fn new(info : SaveDirInfo) -> SaveCacheItem{
        SaveCacheItem{ info, queued : AtomicUsize::new(0) }
    }


    pub(crate) fn clone_dir_info(&self) -> SaveDirInfo{ self.info.clone() }
    pub(crate) fn save_dir(&self) -> &Path{ self.info.save_dir() }
    pub(crate) fn current_src(&self) -> &CurrentSrc{ self.info.current_src() }
    pub(crate) fn hash(&self) -> u128{ self.info.hash() }
    pub(crate) fn clone_src_root(&self) -> RootObject{ self.info.clone_src_root() }
    pub(crate) fn queued(&self) -> usize{ self.queued.load(Ordering::Relaxed) }
    pub(crate) fn queued_atomic(&self) -> &AtomicUsize{ &self.queued }
}