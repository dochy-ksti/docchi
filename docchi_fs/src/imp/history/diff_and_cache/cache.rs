use std::path::{PathBuf};
use crate::error::FsResult;
use crate::imp::history::diff_and_cache::diff_src::DiffSrc;
use crate::imp::history::diff_and_cache::diff_value::DiffValue;
use crate::history::HistoryOptions;

pub(crate) trait Cache<V : DiffValue, S : DiffSrc<V>>{


    fn apply_items_for_save(&mut self, paths : Vec<PathBuf>, op : &HistoryOptions) -> FsResult<S>;
    fn apply_items_for_load(&mut self, load_root : S, paths : Vec<PathBuf>, op : &HistoryOptions) -> FsResult<S>;
    fn set_cache(&mut self, path : PathBuf, item : S, phase : usize) -> FsResult<()>;
}