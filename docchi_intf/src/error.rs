use anyhow::anyhow;
use std::fmt::{Debug, Formatter, Display};
use docchi_core::CoreError;

pub type IntfResult<T> = std::result::Result<T, IntfError>;


pub struct IntfError {
    e : anyhow::Error,
}

impl IntfError {
    pub fn new(e : impl Into<anyhow::Error>) -> Self{ Self{ e : e.into() } }
    pub fn to_string(&self) -> String{ self.e.to_string() }
}

impl Debug for IntfError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.e, f)
    }
}

impl Display for IntfError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.e, f)
    }
}

impl From<CoreError> for IntfError{
    fn from(e : CoreError) -> Self { IntfError::new(e) }
}

impl From<std::io::Error> for IntfError {
    fn from(e : std::io::Error) -> Self {
        Self{ e : anyhow::Error::new(e) }
    }
}

impl From<String> for IntfError {
    fn from(s : String) -> Self {
        Self{ e : anyhow!("{}", s) }
    }
}

impl From<&str> for IntfError {
    fn from(s : &str) -> Self {
        Self{ e : anyhow!("{}", s) }
    }
}

impl Into<anyhow::Error> for IntfError {
    fn into(self) -> anyhow::Error {
        self.e
    }
}