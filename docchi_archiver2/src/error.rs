use std::collections::TryReserveError;
use std::fmt::{Display, Formatter, Debug};
//use std::backtrace::Backtrace;

use anyhow::{anyhow};
use std::time::SystemTimeError;
use std::path::StripPrefixError;
use docchi_compaction::error::ComError;
use std::error::Error;

/// The error type.
///
/// This wraps anyhow::Error. You can get it from Into trait.
///
/// Maybe the source error retrieved from anyhow::Error can be used to determine the cause of the error,
/// but currently, there's no guarantees about the error format.
///
/// I deeply depend on anyhow::Error::backtrace for debugging.
pub struct NouArcError{
    error : anyhow::Error,
}

impl NouArcError{
    pub(crate) fn new(e : impl Error + Send + Sync + 'static) -> Self{ Self{ error : e.into() } }
}

impl Display for NouArcError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.error, f)
    }
}

impl Debug for NouArcError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.error, f)
    }
}

impl Error for NouArcError{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error.source()
    }
}

impl From<ComError> for NouArcError{
    fn from(e: ComError) -> Self {
        Self::new(e)
    }
}

impl From<SystemTimeError> for NouArcError{
    fn from(e : SystemTimeError) -> Self { Self::new(e) }
}

impl From<StripPrefixError> for NouArcError{
    fn from(e: StripPrefixError) -> Self {
        NouArcError::new(e)
    }
}

impl From<std::io::Error> for NouArcError{
    fn from(e : std::io::Error) -> Self { Self::new(e) }
}


impl From<std::string::FromUtf8Error> for NouArcError{
    fn from(e : std::string::FromUtf8Error) -> Self { Self::new(e) }
}

impl From<std::sync::mpsc::RecvError> for NouArcError{
    fn from(e : std::sync::mpsc::RecvError) -> Self { Self::new(e) }
}

impl<T : Send + Sync + 'static> From<std::sync::mpsc::SendError<T>> for NouArcError{
    fn from(e : std::sync::mpsc::SendError<T>) -> Self { Self::new(e) }
}

impl From<snap::Error> for NouArcError{
    fn from(e : snap::Error) -> Self { Self::new(e) }
}

impl From<TryReserveError> for NouArcError{
    fn from(e : TryReserveError) -> Self { Self::new(e) }
}

impl From<&str> for NouArcError{
    fn from(e : &str) -> Self { Self{ error : anyhow!("{}", e) } }
}

impl From<String> for NouArcError{
    fn from(e : String) -> Self { Self{ error : anyhow!("{}", e) } }
}