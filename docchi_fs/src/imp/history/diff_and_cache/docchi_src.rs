use crate::imp::history::diff_and_cache::diff_src::DiffSrc;
use docchi_core::structs::{RootObject};
use crate::error::FsResult;
use crate::imp::history::diff_and_cache::docchi_diff::DocchiDiff;


impl DiffSrc<DocchiDiff> for RootObject{

    fn create_diff(&self, from: &Self) -> FsResult<DocchiDiff> {
        let vec = docchi_diff::get_diff(&from, self)?;
        Ok(DocchiDiff::new(vec))
    }

    fn apply_diff(&mut self, diff: DocchiDiff) -> FsResult<()> {
        docchi_diff::apply_diff(self, &mut diff.slice())?;
        Ok(())
    }
}

