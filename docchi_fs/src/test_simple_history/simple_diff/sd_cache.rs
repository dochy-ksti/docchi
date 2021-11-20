use crate::imp::history::diff_and_cache::cache::Cache;
use crate::test_simple_history::simple_diff::sd_diff::SdDiff;
use crate::test_simple_history::simple_diff::sd_data::SdData;
use std::path::PathBuf;
use crate::error::FsResult;
use crate::history::HistoryOptions;
use crate::imp::history::diff_and_cache::diff_value::DiffValue;
use crate::imp::history::diff_and_cache::diff_src::DiffSrc;
use crate::imp::history::diff_and_cache::open_diff_file_without_metadata::open_diff_file_without_metadata;

pub(crate) struct SdCache{
    size : Option<usize>
}

impl SdCache{
    pub(crate) fn new(size : Option<usize>) -> SdCache{ SdCache{ size } }
    pub(crate) fn create_root(&self) -> SdData{
        SdData::new(self.size)
    }
}

impl Cache<SdDiff, SdData> for SdCache{
    fn apply_items_for_save(&mut self, paths: Vec<PathBuf>, _op: &HistoryOptions) -> FsResult<SdData> {
        let r = self.create_root();
        accumulate_diff(r, paths)
    }

    fn apply_items_for_load(&mut self, load_root: SdData, paths: Vec<PathBuf>, _op: &HistoryOptions) -> FsResult<SdData> {
        accumulate_diff(load_root, paths)
    }

    fn set_cache(&mut self, _path: PathBuf, _item: SdData, _phase: usize) -> FsResult<()> {
        Ok(())
    }
}

pub(crate) fn accumulate_diff<V : DiffValue, S: DiffSrc<V>>(
    mut item : S, paths: Vec<PathBuf>) -> FsResult<S>{

    for path in &paths{
        let mut file = open_diff_file_without_metadata(path)?;
        let diff = V::read_value(&mut file)?;
        item.apply_diff(diff)?;
    }

    Ok(item)
}