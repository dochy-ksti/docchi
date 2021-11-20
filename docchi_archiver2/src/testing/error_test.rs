use std::fmt::{Display, Formatter, Debug};
//use std::backtrace::Backtrace;
use std::error::Error;
use anyhow::anyhow;

pub(crate) struct TestError {
    error : anyhow::Error,
}

impl TestError {
    pub(crate) fn new(e : impl Into<anyhow::Error>) -> Self{ Self{ error : e.into() } }
}

impl Display for TestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.error, f)
    }
}

impl Debug for TestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.error, f)
    }
}

// impl std::error::Error for TestError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         self.error.source()
//     }
//
//     fn backtrace(&self) -> Option<&Backtrace>{
//         Some(self.error.backtrace().into())
//     }
// }

// impl From<NoneError> for TestError {
//     fn from(_: NoneError) -> Self {
//         TestError::new(anyhow!("None Error"))
//     }
// }
// impl From<anyhow::Error> for TestError {
//     fn from(e: anyhow::Error) -> Self {
//         Self::new(e)
//     }
// }

// impl From<&str> for TestError {
//     fn from(e: &str) -> Self {
//         Self::new(anyhow!("{}", e))
//     }
// }

// impl From<String> for TestError {
//     fn from(e: String) -> Self {
//         Self::new(anyhow!("{}", e))
//     }
// }

// impl<E: std::error::Error + Send + Sync + 'static> From<E> for TestError{
//     fn from(e: E) -> Self {
//         Self::new(e)
//     }
// }



// impl<T : std::error::Error + 'static> ErrorConvertable for T{
//     fn into_error(self) -> anyhow::Error {
//         anyhow::Error::new(self)
//     }
// }


fn opt() -> Option<()>{
    None
}

//#[test]
fn test_hoge() -> Result<(), TestError>{
    // match test_inner(){
    //     Ok(_) => Ok(()),
    //     Err(e) =>{
    //         //println!("{}", e);
    //         Err(e)
    //     }
    // }
    match test_inner(){
        Ok(_) => Ok(()),
        Err(e) =>{ println!("{:?}", e); Err(e) },
    }
}

fn test_inner() -> Result<(), TestError>{
    test_inner2()
}

fn test_inner2() -> Result<(), TestError>{

    Ok(())
}