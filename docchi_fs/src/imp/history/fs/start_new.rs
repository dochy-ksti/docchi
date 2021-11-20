use std::path::Path;
use crate::error::FsResult;
//use crate::imp::history::file_name::find_newest_his_file::find_newest_his_file;
use crate::imp::history::fs::write_phase_0::write_phase_0;
use crate::imp::history::fs::first::first;
use crate::imp::history::diff_and_cache::diff_src::DiffSrc;
use crate::imp::history::diff_and_cache::diff_value::DiffValue;
use crate::imp::history::diff_and_cache::cache::Cache;
use crate::imp::history::file_hist::create_file_history::create_file_history;
use crate::imp::history::file_name::file_name_props::FileNameProps;
use crate::imp::history::file_hist::file_history::FileHistory;
use crate::history::HistoryOptions;

pub(crate) fn start_new<V : DiffValue, S: DiffSrc<V>, C : Cache<V,S>>(
    tag : Option<String>,
    diff_src : &S,
    cache : &mut C,
    history_hash_dir: &Path,
    op : &HistoryOptions, cumulative : bool) -> FsResult<FileNameProps>{

    //file history は OS にキャッシュされており、基本的にノーコストで取り出せる、と考えよう。そうしないと単純に出来ない
    let history = create_file_history(history_hash_dir, op.max_phase(), cumulative)?;

    start_new_impl(tag, diff_src, cache, op, history_hash_dir, &history)
}

pub(crate) fn start_new_impl<V : DiffValue, S: DiffSrc<V>, C : Cache<V,S>>(
    tag : Option<String>,
    diff_src : &S,
    cache : &mut C,
    op : &HistoryOptions,
    history_hash_dir: &Path,
    history : &FileHistory) -> FsResult<FileNameProps>{

    if let Some(prop) = history.get_newest_prop(){
        write_phase_0(tag, prop.control() + 1, diff_src, cache, op, history_hash_dir)
    } else{
        first(tag, diff_src, cache, op, history_hash_dir)
    }
}