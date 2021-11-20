use std::io::Read;
use anyhow::{Result};
use with_capacity_safe::vec_with_capacity_safe;

pub(crate) struct Reader<'a, T : Read>{
    read : &'a mut T,
    buf : Vec<u8>,
    len : usize,
}

impl<'a, T : Read> Reader<'a, T>{
    pub fn new(read : &'a mut T) -> Self{ Self{ read, buf : Vec::new(), len : 0 } }
    pub fn read_byte(&mut self) -> Result<u8>{
        let mut buf = [0u8; 1];
        self.read.read_exact(&mut buf)?;
        self.len += 1;
        Ok(buf[0])
    }
    pub fn read(&mut self, bytes : usize) -> Result<&[u8]>{
        self.set_buf_len(bytes);
        self.read.read_exact(&mut self.buf)?;
        self.len += self.buf.len();
        Ok(&self.buf)
    }
    pub fn read_string(&mut self, bytes : usize) -> Result<String>{
        let mut buf : Vec<u8> = vec_with_capacity_safe(bytes)?;
        unsafe{ buf.set_len(bytes); }
        self.read.read_exact(&mut buf)?;
        self.len += buf.len();
        Ok(String::from_utf8(buf)?)
    }
    pub fn read_binary(&mut self, bytes : usize) -> Result<Vec<u8>>{
        let mut buf : Vec<u8> = vec_with_capacity_safe(bytes)?;
        unsafe{ buf.set_len(bytes); }
        self.read.read_exact(&mut buf)?;
        self.len += buf.len();
        Ok(buf)
    }
    pub fn read_binary8(&mut self, len : usize) -> Result<Vec<u64>>{
        let mut buf : Vec<u64> = vec_with_capacity_safe(len)?;
        let mut temp = [0u8; 8];
        for _ in 0..len{
            self.read.read_exact(&mut temp)?;
            self.len += 8;
            buf.push(u64::from_le_bytes(temp));
        }
        Ok(buf)
    }
    pub fn read_binary4(&mut self, len : usize) -> Result<Vec<u32>>{
        let mut buf : Vec<u32> = vec_with_capacity_safe(len)?;
        let mut temp = [0u8; 4];
        for _ in 0..len{
            self.read.read_exact(&mut temp)?;
            self.len += 4;
            buf.push(u32::from_le_bytes(temp));
        }
        Ok(buf)
    }
    pub fn read_binary2(&mut self, len : usize) -> Result<Vec<u16>>{
        let mut buf : Vec<u16> = vec_with_capacity_safe(len)?;
        let mut temp = [0u8; 2];
        for _ in 0..len{
            self.read.read_exact(&mut temp)?;
            self.len += 2;
            buf.push(u16::from_le_bytes(temp));
        }
        Ok(buf)
    }

    pub fn bytes_read(&self) -> usize{ self.len }

    fn set_buf_len(&mut self, len : usize){
        let capacity = self.buf.capacity();
        if len <= capacity{
            unsafe{ self.buf.set_len(len) }
        } else{
            self.buf.reserve(len - self.buf.len());
            unsafe{ self.buf.set_len(len); }
        }
    }
}