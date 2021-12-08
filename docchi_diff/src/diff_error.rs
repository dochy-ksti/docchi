use std::collections::TryReserveError;
use anyhow::anyhow;
use std::fmt::{Debug, Display, Formatter};
use docchi_compaction::kval_enum::KVal;
use std::error::Error;
use docchi_compaction::error::ComError;

pub type DiffResult<T> = std::result::Result<T, DiffError>;

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

///The error type of docchi_diff
pub struct DiffError {
    e : anyhow::Error
}
impl DiffError{
    pub(crate) fn new(e : impl Error + Send + Sync + 'static) -> Self{ Self{ e : e.into() } }
}

impl Display for DiffError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.e, f)
    }
}

impl Debug for DiffError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.e, f)
    }
}

impl Error for DiffError{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.e.source()
    }
}

impl From<TryReserveError> for DiffError{
    fn from(e: TryReserveError) -> Self { DiffError::new(e) }
}

impl From<ComError> for DiffError{
    fn from(e: ComError) -> Self { DiffError::new(e) }
}

impl From<&str> for DiffError{
    fn from(e: &str) -> Self { Self{ e : anyhow!("{}", e) } }
}

impl From<String> for DiffError{
    fn from(e: String) -> Self { Self{ e : anyhow!("{}", e) } }
}

