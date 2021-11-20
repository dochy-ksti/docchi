use crate::imp::history::diff_and_cache::diff_src::DiffSrc;
use docchi_core::structs::{RootObject};
use crate::error::FsResult;
use crate::imp::history::diff_and_cache::docchi_diff::DochyDiff;


impl DiffSrc<DochyDiff> for RootObject{

    fn create_diff(&self, from: &Self) -> FsResult<DochyDiff> {
        let vec = docchi_diff::get_diff(&from, self)?;
        Ok(DochyDiff::new(vec))
    }

    fn apply_diff(&mut self, diff: DochyDiff) -> FsResult<()> {
        docchi_diff::apply_diff(self, &mut diff.slice())?;
        Ok(())
    }
}

