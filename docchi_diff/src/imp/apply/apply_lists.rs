use docchi_core::HashM;
use docchi_core::structs::{ListSabValue, MetaTable, MetaValue};
use crate::diff_error::DiffError;
use crate::imp::structs_read::ListDiffR;
use crate::imp::apply::diff_to_new_list::diff_to_new_list;
use crate::imp::apply::apply_list_diff::apply_list_diff;

pub(crate) fn apply_lists(lists : Vec<(usize, Option<ListDiffR>)>, meta : &MetaTable,
                    r : &mut HashM<String, ListSabValue>) -> Result<(), DiffError>{

    for (id, op_list_diff) in lists{
        let (key, val) = if let Some(v) = meta.get(id){ v } else{
            Err("meta is invalid apply_lists")?
        };

        match val{
            MetaValue::MList(tables) | MetaValue::OptMil(tables) =>{
                if let Some(list_diff) = op_list_diff{
                    if let Some(ListSabValue::Mil(Some(m))) = r.get_mut(key){
                        apply_list_diff(m.list_mut(), list_diff, tables)?;
                    } else {
                        r.insert(key.to_string(), ListSabValue::Mil(
                            Some(diff_to_new_list(list_diff, tables)?)));
                    }
                } else{
                    //ListSabValue::Noneを入れることが可能になっている・・・必要なんだろうか？
                    r.insert(key.to_string(),ListSabValue::Mil(None));
                }
            },
            MetaValue::Param(_) => Err("meta param is invalid apply_lists")?,
        }
    }

    return Ok(());
}