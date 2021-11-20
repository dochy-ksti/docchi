use std::path::Path;
use crate::error::FsResult;
use crate::imp::history::file_hist::file_history::FileHistory;
use crate::imp::history::diff_and_cache::diff_src::DiffSrc;
use crate::imp::history::diff_and_cache::diff_value::DiffValue;
use crate::imp::history::diff_and_cache::cache::Cache;
use crate::imp::history::algo::history_options::{HistoryOptions};
use crate::imp::history::file_name::analyze_file_name::analyze_file_name;
use crate::imp::history::file_hist::ancestors::create_ancestors_paths;


pub(crate) fn load<
    V : DiffValue,
    S : DiffSrc<V>,
    C : Cache<V,S>,
    P : AsRef<Path>,
    Op : AsRef<HistoryOptions>>(diff_file_path: P,
                                history : &FileHistory,
                                root : S,
                                cache : &mut C,
                     opt : Op) -> FsResult<S> {
    let path = diff_file_path.as_ref();
    let opt = opt.as_ref();
    let dir_path = path.parent().ok_or_else(|| format!("No parent found {:?}", path))?;
    let filename = path.file_name().ok_or_else(|| format!("The filename couldn't be found {:?}", path))?.to_string_lossy().to_string();
    let analyzed = analyze_file_name(&filename, Some(opt.max_phase()))
        .ok_or_else(|| format!("invalid file name {}", &filename))?;

    let paths = create_ancestors_paths(history, &analyzed, opt.max_phase(), opt.is_cumulative(), dir_path)?;

    Ok(cache.apply_items_for_load(root, paths,  opt)?)
}