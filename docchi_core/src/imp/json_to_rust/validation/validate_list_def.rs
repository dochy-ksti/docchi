use crate::imp::json_to_rust::names::Names;
use crate::imp::json_to_rust::validation::validate_ref_def::validate_ref_def;
use crate::error::CoreResult;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::list_value::ListDefValue;
use crate::imp::json_to_rust::validation::validate_old_def_mem::validate_old_list_def_mem;
use crate::imp::structs::list_def_obj::ListDefObj;

///ちゃんとDataDef型になっているかどうか、RefDefがちゃんとしてるかどうか、などDefについて調べる
/// Itemについて調べるvalidate_list/data/mut_listとは違う
/// is_mutの場合、InnerMutしか認めない
pub(crate) fn validate_list_def(def : &ListDefObj, root : &RootObject, can_use_old : bool, is_mut : bool, names : &Names) -> CoreResult<()> {
    validate_ref_def(def.refs(), names)?;
    validate_old_list_def_mem(def.old(), def.default(), names)?;

    for (name,_, val) in def.default().iter() {
        match val {
            // ListDefValue::InnerDataDef(d) => {
            //     if is_mut{
            //         Err(format!("{} {} MutList can't have InnerData", names, name))?;
            //     }
            //     validate_list_def(d, root, can_use_old, is_mut,&names.append(name))?;
            // },
            ListDefValue::MilDef(d) => {
                if is_mut == false{
                    Err(format!("{} {} Data/List can't have InnerMutList", names, name))?;
                }
                let names = &names.append(name);
                //validate_compatible(d.list_def(), d.compatible(), root, can_use_old, names)?;
                validate_list_def(d.default(), root, can_use_old, is_mut, names)?;
            },
            ListDefValue::CilDef(d) => {
                if is_mut {
                    Err(format!("{} {} MutList can't have InnerList", names, name))?;
                }
                validate_list_def(d, root, can_use_old, is_mut,&names.append(name))?;
            },
            ListDefValue::Param(_,_) => {},
            //_ => { Err(format!("{} {} can't be defined here", names, name))? }
        }
    }

    Ok(())
}