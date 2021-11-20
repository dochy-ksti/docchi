use crate::common::CurrentSrc;
use docchi_core::structs::RootObject;

#[derive(Debug, Clone)]
pub(crate) struct CurrentSrcInfo{
    current_src : CurrentSrc,
    src_root : RootObject,
    hash : u128,
}

impl CurrentSrcInfo{
    pub(crate) fn new(current_src : CurrentSrc,
                      src_root : RootObject,
                      hash : u128) -> CurrentSrcInfo{
        CurrentSrcInfo{ current_src, src_root, hash, }
    }

    pub fn current_src(&self) -> &CurrentSrc{ &self.current_src }
    pub fn clone_src_root(&self) -> RootObject{ self.src_root.clone() }
    pub fn hash(&self) -> u128{ self.hash }
}