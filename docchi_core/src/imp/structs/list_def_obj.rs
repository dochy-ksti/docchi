//use crate::{HashM};
use crate::imp::structs::list_value::ListDefValue;
use crate::imp::structs::ref_def_obj::RefDefObj;
use std::collections::hash_map::Iter;
use crate::imp::structs::util::hash_m::{HashS, HashM};

#[derive(Debug, Clone, PartialEq)]
pub struct ListDefObj{
    default : Box<ListDefMap>,
    ///RustValueを巨大にしすぎないためにBoxにしてサイズを削る
    refs: Box<RefDefObj>,
    ///oldに設定されたメンバは、defaultでの初期値を覗いてjsonで値を入れられず、プログラムからも_Oldを付けないとアクセスできない
    old : Box<HashS<String>>,

    //meta_table : Box<MetaTable>,
}


impl ListDefObj{
    pub(crate) fn new(default : HashM<String, (usize, ListDefValue)>, refs : RefDefObj, old : HashS<String>) -> ListDefObj{
        let list_def_map = ListDefMap::new(default);
        //let meta_table = MetaTable::from_list_def(&list_def_map);
        ListDefObj{ default : Box::new(list_def_map), refs : Box::new(refs), old : Box::new(old) }
    }
    pub fn default(&self) -> &ListDefMap{ self.default.as_ref() }
    pub fn refs(&self) -> &RefDefObj{ self.refs.as_ref() }
    pub(crate) fn old(&self) -> &HashS<String>{ self.old.as_ref() }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListDefMap{
    map : HashM<String, (usize, ListDefValue)>
}

impl ListDefMap{
    pub(crate) fn new(map : HashM<String, (usize, ListDefValue)>) -> ListDefMap{
        ListDefMap{ map }
    }

    pub fn get(&self, key : &str) -> Option<&ListDefValue>{ self.map.get(key).map(|(_,v)| v) }
    pub fn get_with_id(&self, key : &str) -> Option<(usize, &ListDefValue)>{ self.map.get(key).map(|(k,v)| (*k,v)) }
    pub fn contains_key(&self, key : &str) -> bool{ self.map.contains_key(key) }
    pub(crate) fn iter(&self) -> ListDefMapIter{ ListDefMapIter{ hash_iter : self.map.iter() } }
    pub fn len(&self) -> usize{ self.map.len() }
}

pub struct ListDefMapIter<'a>{
    hash_iter : Iter<'a, String, (usize, ListDefValue)>,
}

impl<'a> Iterator for ListDefMapIter<'a>{
    type Item = (&'a String, usize, &'a ListDefValue);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((key, (id, val))) = self.hash_iter.next(){
            Some((key, *id, val))
        }  else{ None }
    }
}

impl<'a> IntoIterator for &'a ListDefMap{
    type Item = (&'a String, usize, &'a ListDefValue);
    type IntoIter = ListDefMapIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

