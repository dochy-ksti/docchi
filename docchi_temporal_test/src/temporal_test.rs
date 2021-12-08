use std::time::{UNIX_EPOCH};
use anyhow::Result;
use docchi::core::structs::NullOr;

#[test]
fn temporal_test() -> Result<()>{
    let mut v : Vec<u128> = vec![];
    for _i in 0..10 {
        let now = std::time::SystemTime::now();
        let d = now.duration_since(UNIX_EPOCH)?;
        v.push(d.as_nanos());
    }
    println!("{:?}", v);
    println!("{}", u64::MAX);
    Ok(())
}

struct Hoge{
    s : Option<String>
}

impl Hoge{
    pub fn st(&self) -> Option<&str>{ self.s.as_ref().map(|s| s.as_str())}
}

struct Hoge2{
    s : NullOr<String>
}

impl Hoge2{
    pub fn st(&self) -> NullOr<&str>{ self.s.as_ref().map(|s| s.as_str())}
}