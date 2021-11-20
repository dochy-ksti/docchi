use crate::imp::structs_write::{ListDiffW, ListItemDiffEnumW, BS};
use docchi_core::structs::{ListDefObj, LinkedMap, MutItem, MetaTables};
use crate::imp::prepare::compare_items::compare_items;
use crate::imp::prepare::new_item::new_item;

pub(crate) fn get_mlist_diff<'a, 'b>(from : &'a LinkedMap<MutItem>, to : &'b LinkedMap<MutItem>,
                              to_def : &'b ListDefObj, meta : &'b MetaTables) -> Option<ListDiffW<'b>>{

    let mut result : Vec<(u64, ListItemDiffEnumW)> = Vec::new();
    for (&id, from_item) in from {
        if let Some(to_item) = to.get_item(id) {
            if let Some(diff) = compare_items(from_item, to_item, to_def, meta) {
                result.push((id, ListItemDiffEnumW::Modify(diff)));
            }
        } else {
            result.push((id, ListItemDiffEnumW::Delete));
        }
    }

    let mut prev_id : Option<u64> = None;
    for (&id, to_item) in to{
        if from.contains_key(id) == false{
            result.push((id, ListItemDiffEnumW::Create(
                BS{ prev_id, diff : new_item(to_item, to_def, meta) })));
        }
        prev_id = Some(id)
    }

    if result.len() == 0{
        None
    } else {
        Some(ListDiffW::new(result, meta, to.next_id()))
    }
}



