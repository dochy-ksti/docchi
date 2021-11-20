use crate::imp::structs::ref_value::RefValue;
use std::collections::hash_map::Iter;
use crate::imp::structs::util::hash_m::{HashS, HashM};


#[derive(Debug, PartialEq, Clone)]
pub struct RefDefObj {
    refs: Box<RefDefMap>,
    /// Enum とRefの二通りの定義の仕方があり、Enumの場合は Ref のうち一つだけ値があり、ほかは全部nullにしなきゃいけない。
    /// プログラムからはmatch でアクセス出来る。値があるRefをキャストしてゲットする。
    is_enum : bool,
    ///oldに設定されたメンバは、defaultでの初期値を除いてjsonで値を入れられず、プログラムからも_Oldを付けないとアクセスできない
    old : Box<HashS<String>>,
}

impl RefDefObj{
    pub(crate) fn new(refs : HashM<String, (usize, RefValue)>, is_enum : bool, old : HashS<String>) -> RefDefObj{
        let ref_def_map = RefDefMap::new(refs);
        RefDefObj{ refs : Box::new(ref_def_map), is_enum, old : Box::new(old) }
    }
    pub fn refs(&self) -> &RefDefMap{ self.refs.as_ref() }
    pub(crate) fn old(&self) -> &HashS<String>{ self.old.as_ref() }
    pub(crate) fn is_enum(&self) -> bool{ self.is_enum }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RefDefMap{
    map : HashM<String, (usize, RefValue)>
}

impl RefDefMap {
    pub fn new(map: HashM<String, (usize, RefValue)>) -> RefDefMap {
        RefDefMap { map }
    }

    pub fn get(&self, key: &str) -> Option<&RefValue> { self.map.get(key).map(|(_, v)| v) }
    pub fn get_with_id(&self, key: &str) -> Option<(usize, &RefValue)> { self.map.get(key).map(|(k, v)| (*k,v)) }
    pub fn contains_key(&self, key: &str) -> bool { self.map.contains_key(key) }
    pub(crate) fn iter(&self) -> RefDefMapIter { RefDefMapIter { hash_iter: self.map.iter() } }
    pub fn len(&self) -> usize { self.map.len() }
}

pub struct RefDefMapIter<'a> {
    hash_iter: Iter<'a, String, (usize, RefValue)>,
}

impl<'a> Iterator for RefDefMapIter<'a> {
    type Item = (&'a String, usize, &'a RefValue);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((key, (id, val))) = self.hash_iter.next() {
            Some((key, *id, val))
        } else { None }
    }
}

impl<'a> IntoIterator for &'a RefDefMap {
    type Item = (&'a String, usize, &'a RefValue);
    type IntoIter = RefDefMapIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}



