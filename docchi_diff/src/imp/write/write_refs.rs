use docchi_compaction::kval_enum::KVal;
use std::collections::BTreeMap;
use docchi_core::structs::{ MetaTable, Qv, VarType, QvType, MetaValue};
use crate::imp::write::store_ids::{store_ids_ref, StoredIDs};
use docchi_compaction::basic_compaction::{comp_str};
use crate::diff_error::DiffError;
use crate::imp::write::write_store_ids::write_stored_ids;

pub(crate) fn write_refs(refs : &BTreeMap<usize, &Qv<String>>, meta : &MetaTable, r : &mut Vec<KVal>) -> Result<(), DiffError>{
    let ids = store_ids_ref(refs);
    write_stored_ids(&ids, r);
    match ids{
        StoredIDs::Zero =>{
            Ok(())
        },
        _ =>{
            write_refs2(refs, meta, r)?;
            Ok(())
        },
    }
}

pub(crate) fn write_refs2(refs : &BTreeMap<usize, &Qv<String>>, meta : &MetaTable, r : &mut Vec<KVal>) -> Result<(), DiffError> {
    for (&id, &qv) in refs {
        let p = if let Some((_, MetaValue::Param(p))) = meta.get(id) { p } else { unreachable!("invalid meta") };
        match p.var_type() {
            VarType::Normal => {
                write_val(qv, r)?;
            },
            VarType::Nullable => {
                match qv.qv_type() {
                    QvType::Val => {
                        r.push(KVal::Bit(true));
                        write_val(qv, r)?;
                    },
                    QvType::Null => {
                        r.push(KVal::Bit(false));
                    },
                    QvType::Undefined => { Err("invalid undefined")? }
                }
            },
            VarType::Undefiable => {
                match qv.qv_type() {
                    QvType::Val => {
                        r.push(KVal::Bit(true));
                        write_val(qv, r)?;
                    },
                    QvType::Undefined => {
                        r.push(KVal::Bit(false));
                    },
                    QvType::Null => { Err("invalid null")? }
                }
            },
            VarType::UndefNullable => {
                match qv.qv_type() {
                    QvType::Val => {
                        r.push(KVal::Bit(true));
                        write_val(qv, r)?;
                    },
                    QvType::Null => {
                        r.push(KVal::Bit(false));
                        r.push(KVal::Bit(true));
                    },
                    QvType::Undefined => {
                        r.push(KVal::Bit(false));
                        r.push(KVal::Bit(false));
                    },
                }
            }
        }
    }
    Ok(())
}

fn write_val(s : &Qv<String>, r : &mut Vec<KVal>) -> Result<(), DiffError>{
    if let Qv::Val(s) = s {
        r.push(comp_str(s.to_string()));
        Ok(())
    } else{
        Err("must be val write_refs::write_value")?
    }
}