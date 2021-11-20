use std::io::Write;
use docchi_compaction::kval_enum::KVal;
use docchi_compaction::basic_compaction::comp_int;
use crate::error::FsResult;
use docchi_compaction::enc_dec::decode_from_slice::decode_from_slice;
use docchi_compaction::enc_dec::encode::encode                                                                                                                                                                                                                                                                       ;
use with_capacity_safe::vec_with_capacity_safe;

#[derive(Debug)]
pub(crate) struct SdDiff{
    vec : Vec<SdDiffItem>
}

impl SdDiff{
    pub(crate) fn new(vec : Vec<SdDiffItem>) -> SdDiff{
        SdDiff{ vec }
    }

    pub(crate) fn get(&self, index : usize) -> &SdDiffItem{
        &self.vec[index]
    }
    pub(crate) fn len(&self) -> usize{ self.vec.len() }
    pub(crate) fn iter(&self) -> impl Iterator<Item=&SdDiffItem>{
        self.vec.iter()
    }

    pub(crate) fn encode<W : Write>(&self, write : &mut W) -> FsResult<()>{
        let mut vec : Vec<KVal> = vec![];

        vec.push(comp_int(self.len() as i64));
        for item in self.iter(){
            item.write(&mut vec);
        }

        encode(&vec, write)?;
        Ok(())
    }

    pub(crate) fn decode(bytes : &[u8]) -> FsResult<Self>{
        let (vec,_) = decode_from_slice(bytes)?;
        Self::decode_kvals(&vec)
    }

    pub(crate) fn decode_kvals(kvals : &[KVal]) -> FsResult<SdDiff>{
        let mut iter = kvals.iter();
        let len = iter.next()?.as_i64()? as usize;
        let mut vec : Vec<SdDiffItem> = vec_with_capacity_safe(len)?;

        for _ in 0..len{
            vec.push(SdDiffItem::read(&mut iter)?)
        }

        Ok(SdDiff::new(vec))
    }
}

#[derive(Debug)]
pub(crate) struct SdDiffItem{
    index : usize,
    value : u8,
}

impl SdDiffItem{
    pub(crate) fn new(index : usize, value : u8) -> SdDiffItem{
        SdDiffItem{ index, value }
    }

    pub(crate) fn index(&self) -> usize{ self.index }
    pub(crate) fn value(&self) -> u8{ self.value }

    pub(crate) fn write(&self, vec : &mut Vec<KVal>){
        vec.push(comp_int(self.index as i64));
        vec.push(comp_int(self.value as i64));
    }
    pub(crate) fn read<'a>(iter : &mut impl Iterator<Item=&'a KVal>) -> FsResult<SdDiffItem>{
        let index = iter.next()?.as_i64()? as usize;
        let value = iter.next()?.as_i64()? as u8;

        Ok(SdDiffItem{ index, value })
    }
}