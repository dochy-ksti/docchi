use crate::imp::history::diff_and_cache::diff_value::{DiffValue};
use crate::error::FsResult;
use crate::test_simple_history::simple_diff::sd_diff::SdDiff;
use std::io::{Read, Write};
use docchi_compaction::enc_dec::decode::decode;

impl DiffValue for SdDiff{
    fn read_value<R: Read>(read: &mut R) -> FsResult<Self> {
        let (kvals, _) = decode(read)?;
        SdDiff::decode_kvals(&kvals)
    }

    fn write_value<W: Write>(&self, write: &mut W) -> FsResult<()> {
        self.encode(write)
    }
}

