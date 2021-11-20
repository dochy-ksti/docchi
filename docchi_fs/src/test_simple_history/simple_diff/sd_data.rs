//use rand::rngs::ThreadRng;
use crate::test_simple_history::simple_diff::sd_diff::{SdDiffItem, SdDiff};
use crate::error::FsResult;
use anyhow::anyhow;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SdData{
    bytes : Vec<u8>
}

impl SdData{
    pub(crate) fn new(size : Option<usize>) -> SdData{
        SdData{ bytes : std::iter::repeat(0).take(size.unwrap_or(10000)).collect()}
    }

    pub(crate) fn len(&self) -> usize{ self.bytes.len() }
    pub(crate) fn get(&self, index : usize) -> Option<u8>{ self.bytes.get(index).map(|a| *a) }

    pub(crate) fn mutate_randomly(&mut self){
        let mut rng = thread_rng();
        let index = rng.gen_range(0..self.bytes.len());
        let val = rng.gen_range(0..255);
        self.bytes[index] = val;
    }

    pub(crate) fn mutate(&mut self, num : u8) -> FsResult<()>{
        if num == 0{ return Err("num=0")? }
        let mut index = 0;
        loop{
            if *self.bytes.get(index)? == 0{ break; }

            index += 1;
        }
        for n in 0..num{
            *self.bytes.get_mut(index + n as usize)? = num;
        }
        Ok(())
    }
    pub(crate) fn show_states(&self) -> Vec<u8>{
        let b = &self.bytes;
        let mut result : Vec<u8> = vec![];
        let mut index = 0;
        loop{
            let b = b[index];
            if b == 0{
                return result;
            }
            result.push(b);
            index += b as usize;
        }
    }


    pub(crate) fn apply_diff(&mut self, diff : &SdDiff) -> FsResult<()>{
        for item in diff.iter(){
            self.apply_diff_item(item)?
        }
        Ok(())
    }

    fn apply_diff_item(&mut self, diff : &SdDiffItem) -> FsResult<()>{
        let index = diff.index();
        let len = self.bytes.len();
        if index < len {
            self.bytes[index] = diff.value();
            Ok(())
        } else{
            Err(anyhow!("error index {} is out of bound len {}", index, len))?
        }
    }
}

