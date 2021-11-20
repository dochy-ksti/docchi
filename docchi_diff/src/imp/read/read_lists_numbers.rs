use crate::imp::read::read_list::read_list;
use docchi_core::structs::{MetaValue, MetaTable};
use crate::imp::read::reader::Reader;
use crate::imp::structs_read::ListDiffR;
use crate::diff_error::{DiffError, OptToErr};

pub(crate) fn read_lists_numbers(r: &mut Reader, meta: &MetaTable, n: &Vec<usize>,
                      lists: &mut Vec<(usize, Option<ListDiffR>)>) -> Result<(), DiffError>{
    for &id in n {
        if let Some((_, v)) = meta.get(id) {
            match v {
                MetaValue::OptMil(tables) => {
                    if r.read()?.ast_bool()? {
                        lists.push((id, Some(read_list(r, tables)?)));
                    } else {
                        lists.push((id, None));
                    }
                },
                MetaValue::MList(tables) => {
                    lists.push((id, Some(read_list(r, tables)?)));
                },
                MetaValue::Param(_) => {}
            }
        }
    }
    Ok(())
}
