use crate::imp::history::diff_and_cache::diff_value::{DiffValue};
use crate::error::FsResult;
use std::io::{Read, Write};
use docchi_diff::RootDiffR;
use docchi_core::structs::MetaTable;

pub(crate) struct DocchiDiff {
    vec : Vec<u8>
}

impl DiffValue for DocchiDiff {
    fn read_value<R: Read>(read: &mut R) -> FsResult<Self> {
        DocchiDiff::read_value(read)
    }

    fn write_value<W: Write>(&self, write: &mut W) -> FsResult<()> {
        self.write_value(write)
    }
}

impl DocchiDiff {
    pub(crate) fn new(vec : Vec<u8>) -> DocchiDiff { DocchiDiff { vec } }
    pub(crate) fn slice(&self) -> &[u8]{ &self.vec }
    pub(crate) fn read_value<R: Read>(read: &mut R) -> FsResult<Self> {
        let mut vec : Vec<u8> = vec![];
        read.read_to_end(&mut vec)?;
        Ok(DocchiDiff { vec })
    }
    pub(crate) fn write_value<W: Write>(&self, write: &mut W) -> FsResult<()> {
        write.write_all(&self.vec)?;
        Ok(())
    }

    pub(crate) fn prepare(self, meta_table : &MetaTable) -> FsResult<RootDiffR>{
        let (kvals, _) = docchi_compaction::decode_from_slice(self.slice())?;
        Ok(docchi_diff::get_root_diff_r(kvals, meta_table)?)
    }
}
