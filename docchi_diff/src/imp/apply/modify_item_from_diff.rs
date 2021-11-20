use crate::imp::structs_read::ListItemDiffR;
use docchi_core::structs::{MetaTables, MutItem};
use crate::diff_error::DiffError;
use crate::imp::apply::apply_params::apply_params;
use crate::imp::apply::apply_lists::apply_lists;
use crate::imp::apply::apply_refs::apply_refs;

pub(crate) fn modify_item_from_diff(item : &mut MutItem, diff : ListItemDiffR,
                             meta : &MetaTables) -> Result<(), DiffError>{
    let (params, lists, refs) = diff.deconstruct();
    let values = item.values_mut();
    apply_params(params, meta.items(), values)?;
    apply_lists(lists, meta.items(), values)?;

    let ref_hash = item.refs_mut();
    apply_refs(refs, meta.refs(), ref_hash)?;

    Ok(())
}

