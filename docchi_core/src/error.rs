use anyhow::anyhow;
use std::fmt::{Debug, Formatter, Display};
use std::str::Utf8Error;
use docchi_archiver2::NouArcError;

pub type CoreResult<T> = std::result::Result<T, CoreError>;



pub struct CoreError {
    e : anyhow::Error,
}

impl CoreError {
    pub fn new(e : impl Into<anyhow::Error>) -> Self{ Self{ e : e.into() } }
    pub fn to_string(&self) -> String{ self.e.to_string() }
}

impl Debug for CoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.e, f)
    }
}

impl Display for CoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.e, f)
    }
}

impl From<docchi_json5::MyError> for CoreError {
    fn from(e : docchi_json5::MyError) -> Self {
        Self{ e : e.into() }
    }
}

impl From<std::io::Error> for CoreError {
    fn from(e : std::io::Error) -> Self {
        Self{ e : anyhow::Error::new(e) }
    }
}


impl From<String> for CoreError {
    fn from(s : String) -> Self {
        Self{ e : anyhow!("{}", s) }
    }
}

impl From<&str> for CoreError {
    fn from(s : &str) -> Self {
        Self{ e : anyhow!("{}", s) }
    }
}

impl From<Utf8Error> for CoreError{
    fn from(e : Utf8Error) -> Self{ Self::new(e) }
}

impl From<NouArcError> for CoreError{
    fn from(e : NouArcError) -> Self{ Self::new(e) }
}

impl Into<anyhow::Error> for CoreError {
    fn into(self) -> anyhow::Error {
        self.e
    }
}

