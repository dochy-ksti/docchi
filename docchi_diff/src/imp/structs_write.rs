use docchi_core::structs::{RustParam, Qv, MetaTable, MetaTables};
use std::collections::BTreeMap;

#[derive(Debug)]
pub(crate) struct RootDiffW<'a>{
    params : BTreeMap<usize, &'a RustParam>,
    lists : BTreeMap<usize, Option<ListDiffW<'a>>>,
    meta_table : &'a MetaTable,
}
impl<'a> RootDiffW<'a>{
    pub(crate) fn new(params : BTreeMap<usize, &'a RustParam>, lists : BTreeMap<usize, Option<ListDiffW<'a>>>, meta_table : &'a MetaTable) -> RootDiffW<'a>{
        RootDiffW { params, lists, meta_table }
    }
    pub(crate) fn params(&self) -> &BTreeMap<usize, &'a RustParam>{ &self.params }
    pub(crate) fn lists(&self) -> &BTreeMap<usize, Option<ListDiffW<'a>>>{ &self.lists }
    pub(crate) fn meta_table(&self) -> &MetaTable{ self.meta_table }
}

#[derive(Debug)]
pub(crate ) struct ListDiffW<'a>{
    items : Vec<(u64, ListItemDiffEnumW<'a>)>,
    next_id : u64,
    meta : &'a MetaTables,
}

impl<'a> ListDiffW<'a>{
    pub(crate) fn new(items : Vec<(u64, ListItemDiffEnumW<'a>)>, meta : &'a MetaTables, next_id : u64) -> ListDiffW<'a>{
        ListDiffW { items, meta, next_id }
    }
    pub(crate) fn items(&self) -> &Vec<(u64, ListItemDiffEnumW<'a>)>{ &self.items }
    pub(crate) fn meta(&self) -> &MetaTables{ self.meta }
    pub(crate) fn next_id(&self) -> u64{ self.next_id }
}
#[derive(Debug)]
pub(crate) enum ListItemDiffEnumW<'a>{
    Delete,
    Create(BS<'a>),
    Modify(ListItemDiffW<'a>),
}
#[derive(Debug)]
pub(crate) struct BS<'a>{
    pub(crate) prev_id : Option<u64>,
    pub(crate) diff : ListItemDiffW<'a>,
}
#[derive(Debug)]
pub(crate) struct ListItemDiffW<'a>{
    params : BTreeMap<usize, &'a RustParam>,
    refs : BTreeMap<usize, &'a Qv<String>>,
    lists : BTreeMap<usize, Option<ListDiffW<'a>>>,
}

impl<'a> ListItemDiffW<'a>{
    pub(crate) fn new(
        params : BTreeMap<usize, &'a RustParam>,
        refs : BTreeMap<usize, &'a Qv<String>>,
        lists : BTreeMap<usize, Option<ListDiffW<'a>>>) -> ListItemDiffW<'a>{

        ListItemDiffW { params, refs, lists }
    }
    pub(crate) fn params(&self) -> &BTreeMap<usize, &'a RustParam>{ &self.params }
    pub(crate) fn refs(&self) -> &BTreeMap<usize, &'a Qv<String>>{ &self.refs }
    pub(crate) fn lists(&self) -> &BTreeMap<usize, Option<ListDiffW<'a>>>{ &self.lists }
}