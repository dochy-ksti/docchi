use docchi_core::structs::{MutListVal, ListDefObj, MetaTables};
use crate::imp::structs_write::{ListDiffW, ListItemDiffEnumW, BS};
use crate::imp::prepare::new_item::new_item;

pub(crate) fn new_list<'a, 'b>(mil : &'a MutListVal, def : &'a ListDefObj, meta : &'a MetaTables) -> ListDiffW<'a>{
    let mut result : Vec<(u64, ListItemDiffEnumW)> = Vec::new();

    let mut prev_id : Option<u64> = None;
    for (&id, item) in mil.list() {
        result.push((id, ListItemDiffEnumW::Create(
            BS{ prev_id, diff : new_item(item, def, meta) })));
        prev_id = Some(id);
    }

    ListDiffW::new(result, meta, mil.list().next_id())
}