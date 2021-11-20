use std::path::Path;
use crate::error::FsResult;
use crate::imp::history::file_name::calc_filename::calc_filename;
use crate::imp::history::algo::phase_data::PhaseData;
use crate::imp::history::fs::write_phase_file::write_phase_file;
use crate::imp::history::diff_and_cache::diff_src::DiffSrc;
use crate::imp::history::diff_and_cache::diff_value::DiffValue;
use crate::imp::history::diff_and_cache::cache::Cache;
use crate::imp::history::file_name::file_name_props::FileNameProps;
use crate::history::HistoryOptions;

pub(crate) fn write_phase_0<V : DiffValue, S: DiffSrc<V>, C : Cache<V,S>>(
    tag : Option<String>,
    control : u32,
    diff_src: &S,
    cache : &mut C,
    op : &HistoryOptions,
    history_hash_dir: &Path) -> FsResult<FileNameProps>{

    let file_name = calc_filename(tag.as_ref().map(|s| s.as_str()), control, None,&[0]);
    let file_path = history_hash_dir.join(&file_name);

    let initial = cache.apply_items_for_save(vec![], op)?;

    let diff = diff_src.create_diff(&initial)?;
    let mut vec : Vec<u8> = vec![];
    diff.write_value(&mut vec)?;
    let data = PhaseData::new(vec.len() as u64);

    write_phase_file(&data, &file_path, &vec)?;
    cache.set_cache(file_path, diff_src.clone(), 0)?;

    Ok(FileNameProps::from(&file_name)
        .ok_or_else(||format!("The filename couldn't be converted into FileNameProps {:?}", &file_name))?)
}