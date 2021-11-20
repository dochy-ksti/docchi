use std::io::{Write, Read};
use std::time::{SystemTime, Duration};
use crate::error::FsResult;


pub(crate) fn create_time_dat<W : Write>(time : SystemTime, write : &mut W) -> FsResult<()>{
    let std_time = time.duration_since(SystemTime::UNIX_EPOCH)?;
    let secs = std_time.as_secs();
    let nanos = std_time.subsec_nanos();
    write.write_all(&secs.to_le_bytes())?;
    write.write_all(&nanos.to_le_bytes())?;
    Ok(())
}

pub(crate) fn from_time_dat<R : Read>(read : &mut R) -> FsResult<SystemTime>{
    let mut secs = [0 as u8;8];
    read.read_exact(&mut secs)?;
    let mut nanos = [0 as u8; 4];
    read.read_exact(&mut nanos)?;
    let secs = u64::from_le_bytes(secs);
    let nanos =  u32::from_le_bytes(nanos);
    let time = SystemTime::UNIX_EPOCH + Duration::from_secs(secs) + Duration::from_nanos(nanos as u64);
    Ok(time)
}