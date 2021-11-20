use crate::imp::read::reader::Reader;
use docchi_core::structs::{MetaTable, RustParam, MetaValue};
use crate::diff_error::DiffError;
use crate::imp::read::read_param::read_param;

///bitsのフラグが1のところには差分がある。差分がparamならばread_paramする。
///データ保存順序的に、paramが先で、listsはその後に入っているため、listsのとこは一旦スキップする
pub(crate) fn read_params(reader : &mut Reader, bits : u64, meta : &MetaTable, offset : usize, r : &mut Vec<(usize, RustParam)>) -> Result<(), DiffError> {
    let mut flag: u64 = 1;
    let mut index = 0;

    for _ in 0..64 {
        if let Some(p) = read_a_param(reader, bits, flag, index, offset, meta)? {
            r.push((index + offset, p))
        }
        flag <<= 1;
        index += 1;
        if bits < flag { break }
    }
    return Ok(())
}

fn read_a_param(reader : &mut Reader, bits : u64, flag : u64, index : usize, offset : usize, meta : &MetaTable) -> Result<Option<RustParam>, DiffError>{
    if bits & flag != 0{
        if let Some((_,a)) = meta.get(index + offset){
            match a{
                MetaValue::Param(p) =>{
                    return Ok(Some(read_param(p, reader)?));
                },
                _ =>{}
            }
        }
    }
    Ok(None)
}