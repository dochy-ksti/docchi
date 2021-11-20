use crate::imp::read::reader::Reader;
use docchi_core::structs::{MetaTable, MetaValue, VarType, Qv, MetaParam};
use crate::diff_error::{DiffError, OptToErr};
use crate::imp::read::read_store_ids::read_stored_ids;
use crate::imp::write::store_ids::StoredIDs;

pub(crate) fn read_refs(r : &mut Reader, meta : &MetaTable,
                 ss : &mut Vec<(usize, Qv<String>)>) -> Result<(), DiffError> {
    let ids = read_stored_ids(r)?;
    match ids{
        StoredIDs::Zero => return Ok(()),
        StoredIDs::U64(b) =>{
            read_refs_bits(r, b, meta, 0, ss)?;
            Ok(())
        },
        StoredIDs::Bits(b) => {
            let mut offset = 0;
            for &bits in &b{
                read_refs_bits(r, bits, meta, offset, ss)?;
                offset += 64;
            }
            Ok(())
        },
        StoredIDs::Numbers(n) => {
            for idx in n {
                let p = if let Some((_, MetaValue::Param(p))) = meta.get(idx) { p } else {
                    panic!("invalid meta read_a_ref")
                };
                if let Some(p) = read_a_ref(r, p)? {
                    ss.push((idx,p));
                }
            }
            Ok(())
        }
    }
}

pub(crate) fn read_refs_bits(reader : &mut Reader, bits : u64, meta : &MetaTable, offset : usize, r : &mut Vec<(usize, Qv<String>)>) -> Result<(), DiffError> {
    let mut flag: u64 = 1;
    let mut index = 0;

    for _ in 0..64 {
        if bits & flag != 0 {
            let p = if let Some((_, MetaValue::Param(p))) = meta.get(index + offset) { p } else {
                panic!("invalid meta read_a_ref")
            };
            if let Some(p) = read_a_ref(reader, p)? {
                r.push((index + offset, p))
            }
        }
        flag <<= 1;
        index += 1;
        if bits < flag { break }
    }
    return Ok(())
}


fn read_a_ref(r : &mut Reader, p : &MetaParam) -> Result<Option<Qv<String>>, DiffError> {
    match p.var_type() {
        VarType::Normal => {
            Ok(Some(read_val(r)?))
        },
        VarType::Nullable => {
            if r.read()?.ast_bool()? {
                Ok(Some(read_val(r)?))
            } else {
                Ok(Some(Qv::Null))
            }
        },
        VarType::Undefiable => {
            if r.read()?.ast_bool()? {
                Ok(Some(read_val(r)?))
            } else {
                Ok(Some(Qv::Undefined))
            }
        },
        VarType::UndefNullable => {
            if r.read()?.ast_bool()? {
                Ok(Some(read_val(r)?))
            } else if r.read()?.ast_bool()? {
                Ok(Some(Qv::Null))
            } else {
                Ok(Some(Qv::Undefined))
            }
        }
    }
}

fn read_val(r : &mut Reader) -> Result<Qv<String>, DiffError>{
    Ok(Qv::Val(r.read()?.ast_string()?))
}