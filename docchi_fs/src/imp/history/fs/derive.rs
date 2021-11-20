use std::path::Path;
use crate::error::FsResult;
use crate::imp::history::diff_and_cache::diff_src::DiffSrc;
use crate::imp::history::diff_and_cache::diff_value::DiffValue;
use crate::imp::history::diff_and_cache::cache::Cache;
use crate::imp::history::algo::history_options::{HistoryOptions};
use crate::imp::history::file_hist::create_file_history::create_file_history;
use crate::history::FileNameProps;
use crate::imp::history::fs::derive_impl::derive_impl;


pub(crate) fn _derive<
    V : DiffValue,
    S: DiffSrc<V>,
    C : Cache<V,S>,
    P : AsRef<Path>,
    Op : AsRef<HistoryOptions>>(tag : Option<String>,
                     diff_src: &S,
                     cache : &mut C,
                     history_hash_dir: P,
                     from : &FileNameProps,
                     opt: Op) -> FsResult<FileNameProps> {
    let history_hash_dir = history_hash_dir.as_ref();
    let opt = opt.as_ref();
    let history = create_file_history(history_hash_dir, opt.max_phase(), opt.is_cumulative())?;

    derive_impl(tag, diff_src, cache, history_hash_dir, &history, from, opt)
}