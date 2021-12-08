use std::collections::TryReserveError;
use std::fmt::{Display, Formatter, Debug};
//use std::backtrace::Backtrace;
use anyhow::{anyhow};
use std::time::SystemTimeError;
use docchi_compaction::kval_enum::KVal;
use std::error::Error;
use docchi_compaction::error::ComError;

pub type FsResult<T> = Result<T, FsError>;

pub(crate) trait OptToErr{
    fn ast_i64(&self) -> Result<i64, &'static str>;
    fn ast_bool(&self) -> Result<bool, &'static str>;
    fn ast_f64(&self) -> Result<f64, &'static str>;
    fn ast_string(&self) -> Result<String, &'static str>;
}

impl OptToErr for KVal{
    fn ast_i64(&self) -> Result<i64, &'static str> {
        self.as_i64().ok_or("The value is not i64")
    }
    fn ast_bool(&self) -> Result<bool, &'static str> {
        self.as_bool().ok_or("The value is not bool")
    }
    fn ast_f64(&self) -> Result<f64, &'static str> {
        self.as_f64().ok_or("The value is not f64")
    }
    fn ast_string(&self) -> Result<String, &'static str> {
        self.as_string().ok_or("The value is not string")
    }
}

pub struct FsError {
    error : anyhow::Error,
}

impl FsError {
    pub fn new(e : impl Error + Send + Sync + 'static) -> Self{ Self{ error : e.into() } }
}

impl Display for FsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.error, f)
    }
}

impl Debug for FsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.error, f)
    }
}

impl Error for FsError{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error.source()
    }
}

impl From<ComError> for FsError {
    fn from(e: ComError) -> Self {
        Self::new(e)
    }
}

impl From<SystemTimeError> for FsError {
    fn from(e : SystemTimeError) -> Self { Self::new(e) }
}

impl From<std::io::Error> for FsError {
    fn from(e : std::io::Error) -> Self { Self::new(e) }
}

impl From<docchi_archiver2::NouArcError> for FsError{
    fn from(e : docchi_archiver2::NouArcError) -> Self{ Self::new(e) }
}

impl From<docchi_core::CoreError> for FsError{
    fn from(e : docchi_core::CoreError) -> Self{ Self::new(e) }
}

impl From<docchi_diff::DiffError> for FsError{
    fn from(e : docchi_diff::DiffError) -> Self{ Self::new(e) }
}

impl From<TryReserveError> for FsError{
    fn from(e : TryReserveError) -> Self{ Self::new(e) }
}

impl From<&str> for FsError{
    fn from(e : &str) -> Self{ Self{ error :anyhow!("{}", e) } }
}

impl From<String> for FsError{
    fn from(e : String) -> Self{ Self{ error : anyhow!("{}", e) } }
}

//impl From<std::sys_common::poison::PoisonError<Guard>> for FsError{
  //  fn from(e : docchi_diff::diff_error::DiffError) -> Self{ Self::new(e) }
//}