use docchi_core::structs::{LinkedMap, MutItem, MetaTables};
use crate::imp::structs_read::{ListDiffR, ListItemDiffEnumR, CS};
use crate::diff_error::DiffError;
use crate::imp::apply::new_item_from_diff::new_item_from_diff;
use crate::imp::apply::modify_item_from_diff::modify_item_from_diff;

pub(crate) fn apply_list_diff(list : &mut LinkedMap<MutItem>, diff : ListDiffR, meta : &MetaTables)
    -> Result<(), DiffError>{
    let (next_id, vec) = diff.deconstruct();
    list.set_next_id(next_id);
    for (id, diff) in vec{
        match diff{
            ListItemDiffEnumR::Create(CS{ prev_id, diff }) =>{
                if list.insert_first_with_id(id,new_item_from_diff(diff, meta)?) == false{
                    Err("ID is invalid")?
                }
                if let Some(prev_id) = prev_id{
                    if list.move_to_next(prev_id, id) == false{
                        Err(format!("prev_id {} was not found", prev_id))?
                    }
                }
            },
            ListItemDiffEnumR::Delete =>{
                list.remove(id);
            },
            ListItemDiffEnumR::Modify(diff) =>{
                let item = if let Some(item) = list.get_item_mut(id){ item } else{
                    Err("list is invalid apply_list_diff")?
                };
                modify_item_from_diff(item, diff, meta)?;
            }
        }
    }
    Ok(())
}

