use crate::ArcResult;
use std::io::Write;
use docchi_compaction::kval_enum::KVal;
use docchi_compaction::basic_compaction::{comp_str, comp_int};
use std::collections::BTreeMap;

pub(crate) fn write_items<W : Write>(items : BTreeMap<String, Vec<u8>>, writer : &mut W) -> ArcResult<()>{
    let mut kvals : Vec<KVal> = Vec::with_capacity(items.len());
    let mut comps : Vec<Vec<u8>> = Vec::with_capacity(items.len());

    kvals.push(comp_int(items.len() as i64));
    for (path, compressed) in items{
        kvals.push(comp_str(path));
        kvals.push(comp_int(compressed.len() as i64));
        comps.push(compressed);
    }

    docchi_compaction::encode(&kvals, writer)?;

    for comp in &comps{
        writer.write_all(comp)?;
    }

    Ok(())
}