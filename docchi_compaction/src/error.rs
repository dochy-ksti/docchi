use anyhow::anyhow;
use std::fmt::{Debug, Formatter, Display};
use std::error::Error;
use std::collections::TryReserveError;
use std::string::FromUtf8Error;

pub type ComResult<T> = std::result::Result<T, ComError>;


pub struct ComError {
    e : anyhow::Error,
}

impl ComError {
    pub fn new(e : impl Error + Send + Sync + 'static) -> Self{ Self{ e : e.into() } }
    pub fn to_string(&self) -> String{ self.e.to_string() }
}

impl Debug for ComError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.e, f)
    }
}

impl Display for ComError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.e, f)
    }
}

impl Error for ComError{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.e.source()
    }
}

impl From<std::io::Error> for ComError {
    fn from(e : std::io::Error) -> Self {
        Self{ e : anyhow::Error::new(e) }
    }
}

impl From<String> for ComError {
    fn from(s : String) -> Self {
        Self{ e : anyhow!("{}", s) }
    }
}

impl From<&str> for ComError {
    fn from(s : &str) -> Self {
        Self{ e : anyhow!("{}", s) }
    }
}

impl From<TryReserveError> for ComError{
    fn from(e: TryReserveError) -> Self { Self{ e : e.into() } }
}

impl From<FromUtf8Error> for ComError{
    fn from(e: FromUtf8Error) -> Self { Self{ e : e.into() } }
}
