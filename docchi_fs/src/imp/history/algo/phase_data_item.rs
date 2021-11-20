use docchi_compaction::kval_enum::KVal;
use crate::error::FsResult;
use docchi_compaction::basic_compaction;

pub(crate) struct PhaseDataItem{
    my_size : u64,
    descendant_total_size : u64,
    descendant_total_len : usize,
}

impl PhaseDataItem{
    pub(crate) fn new(my_size : u64) -> PhaseDataItem{
        PhaseDataItem{ my_size,
            descendant_total_size : 0,
            descendant_total_len : 0,
        }
    }

    pub(crate) fn add(&mut self, size : u64){
        self.descendant_total_size += size;
        self.descendant_total_len += 1;
    }

    pub(crate) fn my_size(&self) -> u64{ self.my_size }
    pub(crate) fn descendant_total_size(&self) -> u64{ self.descendant_total_size }
    pub(crate) fn descendant_total_len(&self) -> usize{ self.descendant_total_len }

    pub(crate) fn encode(&self, vec : &mut Vec<KVal>){
        vec.push(basic_compaction::comp_int(self.my_size as i64));
        vec.push(basic_compaction::comp_int(self.descendant_total_size as i64));
        vec.push(basic_compaction::comp_int(self.descendant_total_len as i64));
    }

    pub(crate) fn decode<'a>(reader : &mut impl Iterator<Item=&'a KVal>) -> FsResult<PhaseDataItem>{
        let my_size = read_int(reader)? as u64;
        let descendant_total_size = read_int(reader)? as u64;
        let descendant_total_len = read_int(reader)? as usize;
        Ok(PhaseDataItem{ my_size, descendant_total_size, descendant_total_len })
    }
}

fn read_int<'a>(reader : &mut impl Iterator<Item=&'a KVal>) -> FsResult<i64> {
    if let Some(v) = reader.next() {
        if let Some(i) = v.as_i64() {
            return Ok(i);
        }
    }
    Err("PhaseDataItem::decode failed")?
}
