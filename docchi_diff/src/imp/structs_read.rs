use docchi_core::structs::{RustParam, Qv};

pub struct RootDiffR{
    params : Vec<(usize, RustParam)>,
    //Noneはありえないのだが他と同じルーチンを使うとOptionが帰ってくる
    // 変換するのも面倒だし固有ルーチンにするのも大変なのでこれで
    lists : Vec<(usize, Option<ListDiffR>)>,
}
impl RootDiffR{
    pub(crate) fn new(params : Vec<(usize, RustParam)>, lists : Vec<(usize, Option<ListDiffR>)>) -> RootDiffR{
        RootDiffR { params, lists }
    }
    pub(crate) fn default() -> RootDiffR{ RootDiffR{ params : Vec::new(), lists : Vec::new() }}
    pub(crate) fn deconstruct(self) -> (Vec<(usize, RustParam)>, Vec<(usize, Option<ListDiffR>)>){ (self.params, self.lists) }
    //pub(crate) fn params(&self) -> &Vec<(usize, RustParam)>{ &self.params }
    //pub(crate) fn lists(&self) -> &Vec<(usize, ListDiffR)>{ &self.lists }
}


pub(crate) struct ListDiffR{
    items : Vec<(u64, ListItemDiffEnumR)>,
    next_id : u64,
    //meta_table : &'a MetaTable,
    //ref_meta_table : &'a MetaTable,
}

impl ListDiffR{
    pub(crate) fn new(items : Vec<(u64, ListItemDiffEnumR)>, next_id : u64) -> ListDiffR{
        ListDiffR{ items, next_id }
    }
    pub(crate) fn deconstruct(self) -> (u64, Vec<(u64, ListItemDiffEnumR)>){ (self.next_id, self.items) }
}

pub(crate) enum ListItemDiffEnumR{
    Delete,
    Create(CS),
    Modify(ListItemDiffR),
}

pub(crate) struct CS{
    pub(crate) prev_id : Option<u64>,
    pub(crate) diff : ListItemDiffR,
}

pub(crate) struct ListItemDiffR{
    params : Vec<(usize, RustParam)>,
    lists : Vec<(usize, Option<ListDiffR>)>,
    refs : Vec<(usize, Qv<String>)>,
}

impl ListItemDiffR{
    pub(crate) fn new(
        params : Vec<(usize, RustParam)>,
        lists : Vec<(usize, Option<ListDiffR>)>,
        refs : Vec<(usize, Qv<String>)>) -> ListItemDiffR{

        ListItemDiffR{ params, lists, refs, }
    }

//    pub(crate) fn default() -> ListItemDiffR{
//        ListItemDiffR{ params : Vec::new(), lists : Vec::new(), refs : Vec::new() }
//    }
    //pub(crate) fn params(&self) -> &Vec<(usize, RustParam)>{ &self.params }
    //pub(crate) fn lists(&self) -> &Vec<(usize, Option<ListDiffR>)>{ &self.lists }
    //pub(crate) fn refs(&self) -> &Vec<(usize, Qv<String>)>{ &self.refs }

    pub(crate) fn deconstruct(self)
        -> (Vec<(usize, RustParam)>, Vec<(usize, Option<ListDiffR>)>,Vec<(usize, Qv<String>)>){
        (self.params, self.lists, self.refs)
    }
}