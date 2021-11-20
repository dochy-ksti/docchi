use std::fmt::{Display, Formatter, Debug};
//use std::backtrace::Backtrace;
use anyhow::{anyhow};
use std::time::SystemTimeError;
//use std::time::SystemTimeError;

/// The general result type for dochy
pub type DpResult<T> = Result<T, DpError>;

/// The general error type for dochy
pub struct DpError {
    error : anyhow::Error,
}

impl DpError {
    pub fn new(e : impl Into<anyhow::Error>) -> Self{ Self{ error : e.into() } }
}

impl Display for DpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.error, f)
    }
}

impl Debug for DpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.error, f)
    }
}

impl Into<anyhow::Error> for DpError {
    fn into(self) -> anyhow::Error {
        self.error
    }
}
impl From<anyhow::Error> for DpError {
    fn from(e: anyhow::Error) -> Self {
        Self::new(e)
    }
}

impl From<SystemTimeError> for DpError {
    fn from(e : SystemTimeError) -> Self { Self::new(e) }
}

impl From<std::io::Error> for DpError {
    fn from(e : std::io::Error) -> Self { Self::new(e) }
}

impl From<docchi_archiver2::NouArcError> for DpError{
    fn from(e : docchi_archiver2::NouArcError) -> Self{ Self::new(e) }
}

impl From<docchi_core::CoreError> for DpError{
    fn from(e : docchi_core::CoreError) -> Self{ Self::new(e) }
}

impl From<docchi_intf::IntfError> for DpError{
    fn from(e : docchi_intf::IntfError) -> Self{ Self::new(e) }
}

impl From<docchi_diff::DiffError> for DpError{
    fn from(e : docchi_diff::DiffError) -> Self{ Self::new(e) }
}

impl From<docchi_fs::FsError> for DpError{
    fn from(e : docchi_fs::FsError) -> Self{ Self::new(e) }
}


impl From<&str> for DpError{
    fn from(e : &str) -> Self{ Self::new(anyhow!("{}", e)) }
}

impl From<String> for DpError{
    fn from(e : String) -> Self{ Self::new(anyhow!("{}", e)) }
}
