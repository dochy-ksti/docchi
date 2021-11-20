use crate::{HashM};
use crate::imp::structs::root_value::RootValue;

#[derive(Debug, Clone, PartialEq)]
pub struct RootDefObj{
    def : HashM<String, (usize, RootValue)>,
}

impl RootDefObj{
    pub fn new(def : HashM<String, (usize, RootValue)>) -> RootDefObj{
        RootDefObj{ def }
    }
    pub fn def(&self) -> &HashM<String, (usize, RootValue)>{ &self.def }
    ///DefObjは構築後は変更されないのだが、行きがかり上構築時にmodifyが必要になってしまった
    pub(crate) fn def_mut(&mut self) -> &mut HashM<String, (usize, RootValue)>{ &mut self.def }
    pub fn get(&self, name : &str) -> Option<&RootValue>{ self.def.get(name).map(|(_,v)|v) }
    pub fn contains_key(&self, name : &str) -> bool{ self.def.contains_key(name) }
    pub fn len(&self) -> usize{ self.def.len() }
    //pub fn deconstruct(self) -> HashM<String, (usize, RootValue)>{ self.def }
}
