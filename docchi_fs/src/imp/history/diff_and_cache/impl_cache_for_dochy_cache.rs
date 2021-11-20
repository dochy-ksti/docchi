use crate::imp::history::diff_and_cache::cache::Cache;
use std::path::{PathBuf};
use crate::error::FsResult;
use docchi_core::structs::RootObject;
use crate::imp::history::diff_and_cache::docchi_diff::DochyDiff;
use crate::history::HistoryOptions;
use crate::imp::history::diff_and_cache::dochy_cache::DochyCache;


impl Cache<DochyDiff, RootObject> for DochyCache {


    fn apply_items_for_save(&mut self, paths: Vec<PathBuf>, op: &HistoryOptions) -> FsResult<RootObject> {
        self.apply_items_for_save(paths, op)
    }

    fn apply_items_for_load(&mut self, load_root: RootObject, paths: Vec<PathBuf>, op: &HistoryOptions) -> FsResult<RootObject> {
        self.apply_items_for_load(load_root, paths, op)
    }

    fn set_cache(&mut self, path : PathBuf, item: RootObject, phase: usize) -> FsResult<()> {
        self.set_cache(path, item, phase);
        Ok(())
    }
}
