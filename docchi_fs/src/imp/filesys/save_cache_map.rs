use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::common::{CurrentSrc};
use std::path::{PathBuf, Path};
use crate::error::FsResult;
use crate::imp::filesys::save_dir_info::SaveDirInfo;
use crate::imp::filesys::save_cache_item::SaveCacheItem;
use crate::imp::filesys::dochy_mutex::DochyMutex;
use crate::imp::common::current_src::current_src_map::get_current_src_info;
use std::sync::Mutex;


static MAP : Lazy<Mutex<HashMap<PathBuf, Box<(SaveCacheItem, Mutex<()>)>>>> = Lazy::new(||{
    Mutex::new(HashMap::new())
});

pub(crate) fn cache_and_get_info(save_dir : &Path,
                                          current_src : CurrentSrc) -> FsResult<SaveDirInfo>{
    let mut map = MAP.lock().unwrap();
    if let Some(item) = map.get(save_dir){
        let (cache, _) = item.as_ref();
        return Ok(cache.clone_dir_info());
    }
    let info = create_save_dir_info(save_dir, current_src)?;
    map.insert(save_dir.to_path_buf(), Box::new((SaveCacheItem::new(info.clone()), Mutex::new(()))));
    Ok(info)
}

/// Updates current_src cache corresponding to save_dir.
/// All the copies of old info are going to be incorrect.
/// If save_async in the save_dir is not finished, calling this results undefined behavior.
pub unsafe fn force_update_and_get_info_us<P : AsRef<Path>>(save_dir : P,
                                                            current_src : CurrentSrc) -> FsResult<SaveDirInfo>{
    let save_dir = save_dir.as_ref();
    let mut map = MAP.lock().unwrap();
    let info = create_save_dir_info(save_dir, current_src)?;
    map.insert(save_dir.to_path_buf(),
               Box::new((SaveCacheItem::new(info.clone()), Mutex::new(()))));
    Ok(info)
}

fn create_save_dir_info(save_dir : &Path, current_src : CurrentSrc) -> FsResult<SaveDirInfo>{
    let current = get_current_src_info(current_src)?;
    let info = SaveDirInfo::new(save_dir.to_path_buf(),
                                current.current_src().clone(),
                                current.hash(),
                                current.clone_src_root());
    Ok(info)
}

fn get_map_item<'a>(save_dir : &Path) -> Option<&'a (SaveCacheItem, Mutex<()>)>{
    let map = MAP.lock().unwrap();
    let ptr : *const (SaveCacheItem, Mutex<()>) = map.get(save_dir)?.as_ref();
    Some(unsafe{ &*ptr })
}

pub(crate) fn get_mutex<'a>(save_dir : &Path) -> Option<DochyMutex<'a>>{
    let (cache, mutex) = get_map_item(save_dir)?;
    Some(DochyMutex::new(mutex.lock().unwrap(), cache))
}

pub(crate) fn get_cache<'a>(save_dir : &Path) -> Option<&'a SaveCacheItem>{
    let (cache, _mutex) = get_map_item(save_dir)?;
    Some(cache)
}