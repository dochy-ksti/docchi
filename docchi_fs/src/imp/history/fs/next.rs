use std::path::Path;
use crate::error::FsResult;
use crate::imp::history::fs::first::first;
use crate::imp::history::diff_and_cache::diff_src::DiffSrc;
use crate::imp::history::diff_and_cache::diff_value::{DiffValue};
use crate::imp::history::diff_and_cache::cache::Cache;
use crate::imp::history::algo::history_options::{HistoryOptions};
use crate::imp::history::file_hist::create_file_history::create_file_history;
use crate::imp::history::file_name::file_name_props::FileNameProps;
use crate::imp::history::fs::derive_impl::derive_impl;

//testから使われてるんだけど、 _(アンダースコア)消すと文句言われるのなんだかよくわからん
pub(crate) fn _next<
    V : DiffValue,
    S: DiffSrc<V>,
    C : Cache<V,S>,
    P : AsRef<Path>,
    Op : AsRef<HistoryOptions>>(tag : Option<String>,
                                diff_src: &S,
                                cache : &mut C,
                                history_hash_dir: P,
                                opt: Op) -> FsResult<FileNameProps> {

    let history_hash_dir = history_hash_dir.as_ref();
    let opt = opt.as_ref();

    let history = create_file_history(history_hash_dir, opt.max_phase(), opt.cumulative().is_some())?;
    let newest_prop = if let Some(prop) = history.get_newest_prop() {
         prop
    } else {
        return first(tag, diff_src, cache, opt, history_hash_dir);
    };

    derive_impl(tag, diff_src, cache, history_hash_dir, &history, newest_prop, opt)

}