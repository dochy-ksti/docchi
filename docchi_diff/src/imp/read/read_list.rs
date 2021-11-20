use crate::imp::read::reader::Reader;
use crate::diff_error::{DiffError, OptToErr};
use crate::imp::structs_read::{ListItemDiffEnumR, ListItemDiffR, CS, ListDiffR};
use crate::imp::read::read_params::read_params;
use docchi_core::structs::{MetaTables, MetaValue};
use crate::imp::read::read_lists::read_lists;
use crate::imp::read::read_refs::read_refs;
use crate::imp::read::read_store_ids::read_stored_ids;
use crate::imp::write::store_ids::StoredIDs;
use crate::imp::read::read_param::read_param;
use crate::imp::read::read_lists_numbers::read_lists_numbers;
use with_capacity_safe::vec_with_capacity_safe;

pub(crate) fn read_list(r : &mut Reader, meta : &MetaTables)
    -> Result<ListDiffR, DiffError>{

    let len = r.read()?.ast_i64()? as usize;
    let next_id = r.read()?.ast_i64()? as u64;
    let mut vec : Vec<(u64, ListItemDiffEnumR)> = vec_with_capacity_safe(len)?;
    for _ in 0..len{
        let id = r.read()?.ast_i64()?;
        vec.push((id as u64, read_list_item(r, meta)?));
    }
    Ok(ListDiffR::new(vec, next_id))
}

fn read_list_item(r : &mut Reader, meta : &MetaTables) -> Result<ListItemDiffEnumR, DiffError>{
    if r.read()?.ast_bool()?{
        Ok(ListItemDiffEnumR::Modify(read_list_item2(r, meta)?))
    } else if r.read()?.ast_bool()?{
        Ok(ListItemDiffEnumR::Create(read_cs(r, meta)?))
    } else{
        Ok(ListItemDiffEnumR::Delete)
    }
}

fn read_cs(r : &mut Reader, meta : &MetaTables) -> Result<CS, DiffError>{
    let mut prev_id = None;
    if r.read()?.ast_bool()?{
        prev_id = Some(r.read()?.ast_i64()? as u64);
    }
    let diff = read_list_item2(r, meta)?;
    Ok(CS{ prev_id, diff })
}

fn read_list_item2(r : &mut Reader, meta : &MetaTables) -> Result<ListItemDiffR, DiffError>{
    let ids = read_stored_ids(r)?;
    match ids {
        StoredIDs::Zero => {
            let params = Vec::new();
            let lists = Vec::new();
            let mut refs = Vec::new();
            read_refs(r, meta.refs(), &mut refs)?;
            Ok(ListItemDiffR::new(params, lists, refs))
        }
        StoredIDs::U64(bits) => {
            let mut params = Vec::new();
            let mut lists = Vec::new();
            let mut refs = Vec::new();
            read_params(r, bits, meta.items(), 0, &mut params)?;
            read_lists(r, bits, meta.items(), 0, &mut lists)?;
            read_refs(r, meta.refs(), &mut refs)?;
            Ok(ListItemDiffR::new(params, lists, refs))
        },
        StoredIDs::Bits(bits) => {
            let mut params = Vec::new();
            let mut lists = Vec::new();
            let mut refs = Vec::new();
            let mut offset = 0;
            for &b in &bits {
                read_params(r, b, meta.items(), offset, &mut params)?;
                offset += 64;
            }
            offset = 0;
            for &b in &bits {
                read_lists(r, b, meta.items(), offset, &mut lists)?;
                offset += 64;
            }
            read_refs(r, meta.refs(), &mut refs)?;
            Ok(ListItemDiffR::new(params, lists, refs))
        },
        StoredIDs::Numbers(n) => {
            let mut params = Vec::new();
            let mut lists = Vec::new();
            let mut refs = Vec::new();

            for &id in &n{
                if let Some((_,MetaValue::Param(p))) = meta.items().get(id){
                    params.push((id, read_param(p, r)?));
                } else {
                    Err("invalid meta read_list_item2")?
                }
            }
            read_lists_numbers(r, meta.items(), &n, &mut lists)?;
            read_refs(r, meta.refs(), &mut refs)?;
            Ok(ListItemDiffR::new(params, lists, refs))
        }
    }
}
