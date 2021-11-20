use docchi_compaction::kval_enum::KVal;
use crate::imp::write::store_ids::StoredIDs;
use docchi_compaction::basic_compaction::{comp_int};

pub(crate) fn write_stored_ids(s : &StoredIDs, r : &mut Vec<KVal>){
    match s {
        StoredIDs::Zero => {
            r.push(KVal::Bit(true));
            r.push(KVal::Bit(false));
        },
        StoredIDs::U64(b) => {
            r.push(KVal::Bit(true));
            r.push(comp_int(*b as i64));
        },
        StoredIDs::Bits(b) => {
            r.push(KVal::Bit(false));
            r.push(KVal::Bit(true));
            r.push(comp_int(b.len() as i64));
            for &s in b {
                r.push(comp_int(s as i64));
            }
        },
        StoredIDs::Numbers(n) => {
            r.push(KVal::Bit(false));
            r.push(KVal::Bit(false));
            r.push(comp_int(n.len() as i64));
            for &s in n {
                r.push(comp_int(s as i64));
            }
        }
    }
}