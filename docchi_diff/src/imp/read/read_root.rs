use crate::imp::read::reader::Reader;
use docchi_core::structs::{MetaTable, MetaValue};
use crate::diff_error::DiffError;
use crate::imp::structs_read::{RootDiffR};
use crate::imp::read::read_params::read_params;
use crate::imp::read::read_lists::read_lists;
use crate::imp::read::read_param::read_param;
use crate::imp::read::read_store_ids::read_stored_ids;
use crate::imp::write::store_ids::StoredIDs;
use crate::imp::read::read_lists_numbers::read_lists_numbers;

pub(crate) fn read_root(r : &mut Reader, meta : &MetaTable) -> Result<RootDiffR, DiffError>{
    let ids = read_stored_ids(r)?;
    match ids{
        StoredIDs::Zero => return Ok(RootDiffR::default()),
        StoredIDs::U64(b) =>{
            let mut params = Vec::new();
            let mut lists = Vec::new();
            read_params(r, b, meta, 0, &mut params)?;
            read_lists(r, b, meta, 0, &mut lists)?;
            Ok(RootDiffR::new(params, lists))
        },
        StoredIDs::Bits(b) => {
            let mut params = Vec::new();
            let mut lists = Vec::new();
            let mut offset = 0;
            for &bits in &b{
                read_params(r, bits, meta, offset, &mut params)?;
                offset += 64;
            }
            offset = 0;
            for &bits in &b{
                read_lists(r, bits, meta, offset, &mut lists)?;
                offset += 64;
            }
            Ok(RootDiffR::new(params, lists))
        },
        StoredIDs::Numbers(n) => {
            let mut params = Vec::new();
            let mut lists = Vec::new();
            for &id in &n{
                if let Some((_,MetaValue::Param(p))) = meta.get(id){
                    params.push((id, read_param(p, r)?));
                }
            }
            read_lists_numbers(r, meta, &n, &mut lists)?;
            Ok(RootDiffR::new(params, lists))
        }
    }
}
