use crate::imp::structs_write::{ListDiffW, ListItemDiffEnumW, BS, ListItemDiffW};
use docchi_compaction::kval_enum::KVal;
use crate::imp::write::store_ids::{store_ids, StoredIDs};
use crate::imp::write::write_root::write_params;
use docchi_core::structs::{MetaTable, MetaValue, MetaTables};
use docchi_compaction::basic_compaction::comp_int;
use std::collections::BTreeMap;
use crate::imp::write::write_refs::write_refs;
use crate::diff_error::DiffError;
use crate::imp::write::write_store_ids::write_stored_ids;


pub(crate) fn write_list(l : &ListDiffW, r : &mut Vec<KVal>) -> Result<(), DiffError>{
    let items = l.items();
    let meta = l.meta();
    r.push(comp_int(items.len() as i64));
    r.push(comp_int(l.next_id() as i64));
    for (id,diff) in items{
        r.push(comp_int(*id as i64));
        write_list_item(diff, meta, r)?
    }
    Ok(())
}

fn write_list_item(item : &ListItemDiffEnumW, meta : &MetaTables, r : &mut Vec<KVal>) -> Result<(), DiffError>{
    match item{
        ListItemDiffEnumW::Modify(ld) =>{
            r.push(KVal::Bit(true));
            write_list_item2(ld, meta, r)
        },
        ListItemDiffEnumW::Create(bs) =>{
            r.push(KVal::Bit(false));
            r.push(KVal::Bit(true));
            write_bs(bs, meta, r)
        },
        ListItemDiffEnumW::Delete =>{
            r.push(KVal::Bit(false));
            r.push(KVal::Bit(false));
            Ok(())
        }
    }
}

fn write_bs(bs : &BS, meta : &MetaTables, r : &mut Vec<KVal>) -> Result<(), DiffError>{
    if let Some(prev_id) = bs.prev_id{
        r.push(KVal::Bit(true));
        r.push(comp_int(prev_id as i64));
    } else{
        r.push(KVal::Bit(false));
    }
    write_list_item2(&bs.diff, meta, r)
}

fn write_list_item2(ld : &ListItemDiffW, meta : &MetaTables, r : &mut Vec<KVal>) -> Result<(), DiffError>{
    let params = ld.params();
    let lists = ld.lists();

    let ids = store_ids(params, lists);
    write_stored_ids(&ids, r);
    match ids{
        StoredIDs::Zero =>{
            write_refs(ld.refs(), meta.refs(), r)?;
            Ok(())
        },
        _ =>{
            write_params(params, meta.items(), r)?;
            write_ld_lists(lists, meta.items(), r)?;
            write_refs(ld.refs(), meta.refs(), r)?;
            Ok(())
        },
    }
}

pub(crate) fn write_ld_lists(lists : &BTreeMap<usize, Option<ListDiffW>>, meta : &MetaTable, r : &mut Vec<KVal>) -> Result<(), DiffError> {
    for (&id, op) in lists {
        let mv = if let Some((_, mv)) = meta.get(id) { mv } else { unreachable!("meta is invalid") };
        match mv {
            MetaValue::MList(_tables) => {
                if let Some(op) = op.as_ref() {
                    write_list(op, r)?
                } else {
                    Err("invalid undefined value write_list::write_ld_lists")?
                }
            },
            MetaValue::OptMil(_tables) => {
                if let Some(l) = op {
                    r.push(KVal::Bit(true));
                    write_list(l, r)?
                } else {
                    r.push(KVal::Bit(false));
                }
            },
            MetaValue::Param(_) => {
                panic!("MetaTable is not valid, write_ld_lists");
            }
        }
    }
    Ok(())
}
