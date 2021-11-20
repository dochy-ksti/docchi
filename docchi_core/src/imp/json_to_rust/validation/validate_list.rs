use crate::error::CoreResult;
use crate::imp::json_to_rust::names::Names;
use crate::imp::json_to_rust::validation::validate_list_item::validate_list_item;
use crate::imp::structs::rust_list::ConstItem;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::list_def_obj::ListDefObj;

pub(crate) fn validate_const_list(def : &ListDefObj, data_vec : &Vec<ConstItem>, root : &RootObject, can_use_old: bool, names : &Names) -> CoreResult<()>{
    for (idx, val) in data_vec.iter().enumerate(){
        let idx = format!("#{}", idx);
        let names = &names.append(&idx);
        validate_list_item(def, val.values(), val.refs(), root, can_use_old, names)?;
    }
    return Ok(());
}