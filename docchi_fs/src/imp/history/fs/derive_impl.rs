use std::path::Path;
use crate::error::FsResult;
use crate::imp::history::algo::phase_data::PhaseData;
use crate::imp::history::algo::calc_next_phase::calc_next_phase;
use crate::imp::history::fs::write_phase_file::write_phase_file;
use crate::imp::history::diff_and_cache::diff_src::DiffSrc;
use crate::imp::history::diff_and_cache::diff_value::DiffValue;
use crate::imp::history::diff_and_cache::cache::Cache;
use crate::imp::history::algo::history_options::{HistoryOptions};
use crate::history::FileNameProps;
use crate::imp::history::file_hist::file_history::FileHistory;
use crate::imp::history::file_hist::ancestors::{calc_ancestors_paths, create_ancestors, create_dependencies};


pub(crate) fn derive_impl<
    V : DiffValue,
    S: DiffSrc<V>,
    C : Cache<V,S>,
    P : AsRef<Path>,
    Op : AsRef<HistoryOptions>>(tag : Option<String>,
                                diff_src: &S,
                                cache : &mut C,
                                history_hash_dir: P,
                                history : &FileHistory,
                                from : &FileNameProps,
                                opt: Op) -> FsResult<FileNameProps> {
    let options = opt.as_ref();
    let history_hash_dir = history_hash_dir.as_ref();
    let from_file_path = history_hash_dir.join(from.calc_filename());

    let np = if let Some(np) = history.get_newest_prop(){
        np
    } else{
        Err(format!("{} couldn't be found", from.calc_filename()))?
    };
    let next_ctl = if np == from{ np.control() } else{ np.control() + 1 };

    let mut file = std::fs::File::open(&from_file_path)?;
    let (decoded, _) = docchi_compaction::enc_dec::decode::decode(&mut file)?;
    let mut data = PhaseData::decode(&decoded)?;
    let next_phase = calc_next_phase(&data, options);


    let ancestors1 = create_ancestors(&history, &from, options.max_phase(), options.is_cumulative())?;
    let (ancestors, next_props) = create_dependencies(&ancestors1, next_phase, next_ctl, tag, options.max_phase(), options.is_cumulative())?;

    let paths = calc_ancestors_paths(&ancestors, history_hash_dir);

    let composed = cache.apply_items_for_save(paths,options)?;
    let diff = diff_src.create_diff(&composed)?;

    let mut vec: Vec<u8> = vec![];
    diff.write_value(&mut vec)?;

    //ファイルに書き込む前にlenを求める必要がある
    data.pop_and_push(next_phase, vec.len() as u64);

    let next_file_path = history_hash_dir.join(&next_props.calc_filename());

    write_phase_file(&data, &next_file_path, &vec)?;
    cache.set_cache(next_file_path, diff_src.clone(), next_phase)?;

    Ok(next_props)
}