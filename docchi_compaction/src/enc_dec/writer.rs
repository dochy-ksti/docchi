use std::io::Write;
use anyhow::{Result};

pub(crate) struct Writer<'a, T : Write>{
    write : &'a mut T,
    len : usize,
}

impl<'a, T : Write> Writer<'a, T>{
    pub fn new(write : &'a mut T) -> Self{ Self{ write, len : 0 } }
    pub fn write_byte(&mut self, byte : u8) -> Result<usize>{
        self.write.write_all(&[byte])?;
        self.len += 1;
        Ok(1)
    }
    pub fn write(&mut self, bytes : &[u8]) -> Result<usize>{
        self.write.write_all(bytes)?;
        self.len += bytes.len();
        Ok(bytes.len())
    }
    pub fn bytes_written(&self) -> usize{ self.len }

    //やる必要ある？？？
    // pub fn flush(&mut self) -> Result<()>{
    //     self.write.flush()?;
    //     Ok(())
    // }
}