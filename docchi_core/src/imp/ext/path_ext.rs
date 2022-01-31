use crate::CoreResult;
use std::path::Path;
use std::ffi::OsStr;

pub(crate) trait PathExt{
    fn file_stem_ex(&self) -> CoreResult<&OsStr>;
    fn file_name_ex(&self) -> CoreResult<&OsStr>;
}

impl PathExt for Path{
    fn file_stem_ex(&self) -> CoreResult<&OsStr> {
        Ok(self.file_stem().ok_or_else(||format!("Invalid File Stem {:?}", self))?)
    }
    fn file_name_ex(&self) -> CoreResult<&OsStr> {
        Ok(self.file_name().ok_or_else(||format!("Invalid File Name {:?}", self))?)
    }
}