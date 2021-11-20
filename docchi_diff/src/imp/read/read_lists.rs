use crate::imp::read::reader::Reader;
use docchi_core::structs::{MetaTable, MetaValue};
use crate::imp::structs_read::ListDiffR;
use crate::diff_error::{DiffError, OptToErr};
use crate::imp::read::read_list::read_list;

pub(crate) fn read_lists(reader : &mut Reader, bits : u64, meta : &MetaTable,
                  offset : usize, r : &mut Vec<(usize, Option<ListDiffR>)>) -> Result<(), DiffError> {
    let mut flag: u64 = 1;
    let mut index = 0;
    for _ in 0..64 {
        if bits & flag != 0 {
            if let ReadAList::Result(diff) = read_a_list(reader, index, offset, meta)?{
                r.push((index + offset, diff));
            }
        }
        flag <<= 1;
        index += 1;

        if bits < flag { break }
    }
    return Ok(())
}

enum ReadAList{
    Skip,
    Result(Option<ListDiffR>),
}
fn read_a_list(r : &mut Reader, index : usize, offset : usize,
                meta : &MetaTable) -> Result<ReadAList, DiffError> {
    if let Some((_, a)) = meta.get(index + offset) {
        match a {
            MetaValue::MList(tables) => {
                Ok(ReadAList::Result(Some(read_list(r, tables)?)))
            },
            MetaValue::OptMil(tables) => {
                if r.read()?.ast_bool()?{
                    Ok(ReadAList::Result(Some(read_list(r, tables)?)))
                } else{
                    Ok(ReadAList::Result(None))
                }
            }
            MetaValue::Param(_p) => { Ok(ReadAList::Skip)},
        }
    } else{
        unreachable!("invalid meta read_a_list")
    }
}