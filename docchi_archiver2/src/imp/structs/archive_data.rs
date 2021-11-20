use std::collections::BTreeMap;
use std::sync::Arc;

pub struct ArchiveData<T : Send + 'static>{
    btree : BTreeMap<String, ArchiveDataItem<T>>,
    hash : u128,
}

pub struct ArchiveDataItem<T : Send + 'static>{
    converted_data : T,
    raw_data : Arc<Vec<u8>>,
}

impl<T : Send + 'static> ArchiveData<T>{
    pub fn new(btree : BTreeMap<String, ArchiveDataItem<T>>,
               hash : u128) -> ArchiveData<T>{
        ArchiveData{ btree, hash }
    }

    pub fn btree(&self) -> &BTreeMap<String, ArchiveDataItem<T>>{ &self.btree }
    pub fn hash(&self) -> u128{ self.hash }
    pub fn deconstruct(self) -> BTreeMap<String, ArchiveDataItem<T>>{ self.btree }
}

impl<T : Send + 'static> ArchiveDataItem<T>{
    pub fn new(converted_data : T,
               raw_data : Arc<Vec<u8>>) -> ArchiveDataItem<T>{
        ArchiveDataItem{
            converted_data,
            raw_data,
        }
    }

    pub fn converted_data(&self) -> &T{ &self.converted_data }
    pub fn raw_data(&self) -> &Arc<Vec<u8>>{ &self.raw_data }
    pub fn deconstruct(self) -> (T, Arc<Vec<u8>>){ (self.converted_data, self.raw_data) }
}