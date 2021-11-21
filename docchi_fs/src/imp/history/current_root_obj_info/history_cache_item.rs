use std::sync::atomic::{AtomicUsize, Ordering};
use crate::common::{CurrentSrc};
use crate::history::HistoryOptions;
use docchi_core::structs::RootObject;
use crate::imp::history::current_root_obj_info::current_root_obj_info::CurrentRootObjInfo;
use crate::imp::history::diff_and_cache::docchi_cache::DocchiCache;
use crate::imp::history::current_root_obj_info::fifo_thread::FifoThread;
use std::sync::Mutex;

pub(crate) struct HistoryCacheItem {
    peekable : PeekableCacheInfo,
    synced : Mutex<SyncedItem>,
    fifo_thread : FifoThread
}

impl HistoryCacheItem{
    pub(crate) fn new(peekable : PeekableCacheInfo, synced : Mutex<SyncedItem>) -> HistoryCacheItem{
        HistoryCacheItem{ peekable, synced, fifo_thread : FifoThread::new() }
    }
    pub(crate) fn peekable(&self) -> &PeekableCacheInfo{ &self.peekable }
    pub(crate) fn synced(&self) -> &Mutex<SyncedItem>{ &self.synced }
    pub(crate) fn fifo_thread(&self) -> &FifoThread{ &self.fifo_thread }
}

/// info gettable without accessing mutex
#[derive(Debug)]
pub struct PeekableCacheInfo{
    queued : AtomicUsize,
    current_src : CurrentSrc,
    hash : u128,
    history_options : HistoryOptions,
    src_root : RootObject,
}

impl PeekableCacheInfo{
    pub(crate) fn new(current_src : CurrentSrc,
                      hash : u128,
                      history_options : HistoryOptions,
                      src_root : RootObject) -> PeekableCacheInfo{

        PeekableCacheInfo{ queued : AtomicUsize::new(0), current_src, hash, history_options, src_root }
    }
    pub fn queued_atomic(&self) -> &AtomicUsize{ &self.queued }
    pub fn queued(&self) -> usize{ self.queued.load(Ordering::Relaxed) }
    pub fn current_src(&self) -> &CurrentSrc{ &self.current_src }
    pub fn hash(&self) -> u128{ self.hash }
    pub fn history_options(&self) -> &HistoryOptions{ &self.history_options }
    pub fn clone_src_root(&self) -> RootObject{ self.src_root.clone() }
}

pub(crate) struct SyncedItem{
    cache : DocchiCache,
    current_root : Option<CurrentRootObjInfo>,
}

impl SyncedItem{
    pub(crate) fn new(cache : DocchiCache,
                      current_root : Option<CurrentRootObjInfo>) -> SyncedItem{
        SyncedItem{ cache, current_root }
    }
    pub(crate) fn muts(&mut self) -> (&mut DocchiCache, &mut Option<CurrentRootObjInfo>){
        (&mut self.cache, &mut self.current_root)
    }
}