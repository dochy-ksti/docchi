use crate::error::FsResult;
use docchi_core::structs::RootObject;
use crate::imp::history::fs::load::load;
use crate::history::{FileHistory, HistoryOptions};
use crate::imp::history::file_name::file_name_props::FileNameProps;
use std::path::Path;
use crate::imp::common::path::hash_dir_path::hash_dir_path;
use docchi_core::{ adjust_versions};
use crate::imp::history::file_hist::history_file_data::HistoryFileData;
use crate::imp::history::current_root_obj_info::history_cache_map::{get_mutex};
use crate::imp::history::history_info::HistoryInfo;
use crate::imp::history::current_root_obj_info::current_root_obj_info::CurrentRootObjInfo;
use crate::imp::history::diff_and_cache::docchi_cache::DocchiCache;

/// Loads a history file.
pub fn load_history_file(history_info : &HistoryInfo,
                         props : &FileNameProps,
                         history : &FileHistory,
                         validation : bool) -> FsResult<RootObject> {

    let mut guard = get_mutex(history_info.history_dir()).unwrap();
    let c = guard.peekable();
    let hash = c.hash();
    let op = c.history_options().clone();
    let (cache,h) = guard.muts();
    match load_impl(history_info.history_dir(), hash,
                    props, history, cache, &op, validation){
        Ok(root) =>{
            *h = Some(CurrentRootObjInfo::new(root.id(), props.clone()));
            Ok(root)
        },
        Err(e) => Err(e),
    }
}

fn load_impl<P : AsRef<Path>>(history_dir : P,
                              hash : u128,
                              props : &FileNameProps,
                              history : &FileHistory,
                              cache : &mut DocchiCache,
                              op : &HistoryOptions,
                              validation : bool) -> FsResult<RootObject> {
    let dir = history_dir.as_ref();
    let hash_dir = hash_dir_path(dir, hash);
    let file_path = hash_dir.join(props.calc_filename());

    if cache.hash() != hash{
        let root = cache.get_or_create_hash_root(dir, hash)?;
        let loaded = load(&file_path, history, root, cache,  op)?;
        let adjusted = adjust_versions(cache.clone_src_root(), loaded, validation)?;
        Ok(adjusted)
    } else{
        let root = cache.clone_src_root();
        let loaded = load(&file_path, history, root, cache,op)?;
        Ok(loaded)
    }
}

pub fn load_history_file_data(history_info : &HistoryInfo,
                              data : &HistoryFileData,
                              validation : bool) -> FsResult<RootObject> {
    load_history_file(history_info, data.props(), data.history(), validation)
}