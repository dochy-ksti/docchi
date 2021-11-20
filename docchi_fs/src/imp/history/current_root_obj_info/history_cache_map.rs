use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::common::{CurrentSrc};
use crate::history::{HistoryOptions, PeekableCacheInfo};
use std::path::{PathBuf, Path};
use crate::error::FsResult;
use crate::imp::history::history_info::HistoryInfo;
use crate::imp::history::current_root_obj_info::current_root_obj_info::CurrentRootObjInfo;
use crate::imp::history::current_root_obj_info::history_cache_item::{SyncedItem, HistoryCacheItem};
use crate::imp::history::current_root_obj_info::mutex_g::MutexG;
use crate::imp::history::diff_and_cache::dochy_cache::DochyCache;
use crate::imp::history::current_root_obj_info::fifo_thread::FifoThread;
use std::sync::{Mutex, MutexGuard};


static MAP : Lazy<Mutex<HashMap<PathBuf, Box<HistoryCacheItem>>>> = Lazy::new(||{
    Mutex::new(HashMap::new())
});

pub(crate) fn init_dochy_cache(history_dir : &Path, current_src : CurrentSrc, op : &HistoryOptions) -> FsResult<HistoryInfo>{
    let map = MAP.lock().unwrap();
    if let Some(item) = map.get(history_dir){
        if item.peekable().current_src() != &current_src{
            Err(format!("Source alternation while running is not supported {:?}", history_dir))?
        } else if item.peekable().history_options() != op{
            Err(format!("Changing HistoryOptions while running is not supported {:?}", history_dir))?
        } else{
            return Ok(HistoryInfo::new(history_dir.to_path_buf()));
        }
    }
    init_dochy_cache_impl(map, history_dir, current_src, op)
}

/// The backdoor to change DochySrc while running.
/// References of PeekableCacheInfo for the history_dir will corrupt
/// If save / load in the history_dir is running, calling this results undefined behavior
pub unsafe fn init_dochy_cache_us(history_dir : &Path,
                                  current_src : CurrentSrc,
                                  op : &HistoryOptions) -> FsResult<HistoryInfo>{
    let map = MAP.lock().unwrap();
    init_dochy_cache_impl(map, history_dir, current_src, op)
}

fn init_dochy_cache_impl(mut map : MutexGuard<HashMap<PathBuf, Box<HistoryCacheItem>>>,
                         history_dir : &Path,
                         current_src : CurrentSrc,
                         op : &HistoryOptions) -> FsResult<HistoryInfo>{

    let cache = DochyCache::new(current_src.clone())?;
    let hash = cache.hash();
    let src_root = cache.clone_src_root();
    map.insert(history_dir.to_path_buf(), Box::new(HistoryCacheItem::new(
        PeekableCacheInfo::new(
            current_src,
            hash,
            op.clone(),
            src_root),
        Mutex::new(SyncedItem::new(
            cache,
            None))
    )));
    return Ok(HistoryInfo::new(history_dir.to_path_buf()));
}

///unbound life time だが、Boxのアドレスは固定なので安全なはず
pub(crate) fn get_map_item<'a>(history_dir : &Path) -> Option<&'a HistoryCacheItem>{
    let ptr: *const HistoryCacheItem = {
        let map = MAP.lock().unwrap();
        let b = map.get(history_dir)?;
        b.as_ref()
    };
    Some(unsafe{ &*ptr })
}


pub(crate) fn get_mutex<'a>(history_dir : &Path) -> Option<MutexG<'a>>{
    let item = get_map_item(history_dir)?;
    let peekable_info = item.peekable();
    let guard = item.synced().lock().unwrap();

    Some(MutexG::new(guard, peekable_info))
}

/// You can peek the file to be derived in the next save, but the Mutex is needed for save and load.
/// If you call save or load while the MutexGuard is alive, deadlock occurs.
pub fn get_peekable_info<'a>(history_info : &HistoryInfo) -> &'a PeekableCacheInfo{
    let item = get_map_item(history_info.history_dir()).unwrap();
    item.peekable()
}

pub(crate) fn get_fifo_thread<'a>(history_info : &HistoryInfo) -> Option<&'a FifoThread>{
    let item = get_map_item(history_info.history_dir())?;
    Some(item.fifo_thread())
}


/// During save and load, the RootObject's ID and the selected file is recorded. If you use the same RootObject in the next save,
/// the file to be derived is automatically selected by the system.
///
/// This is the backdoor. You can set the ID and file info and designate the file to be derived in the next save.
/// Arbitrary deriving is not supported. You must derive from an older state of the RootObject.
///
/// Calling this function before the MutexGuard is dropped results deadlock.
pub fn set_current_root_obj_info(history_info : &HistoryInfo, current_root_obj_info : Option<CurrentRootObjInfo>){
    let m = get_map_item(history_info.history_dir()).unwrap();
    let mut s = m.synced().lock().unwrap();
    let (_,h) = s.muts();
     *h = current_root_obj_info;
}