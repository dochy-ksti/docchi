// use crate::structs::{ListDefObj, MutListVal, ListDefValue, MutListDef};
// use crate::imp::structs::list_sab_value::ListSabValue;
//
// pub(crate) fn set_empty_mils(def : &ListDefObj, list : &mut MutListVal){
//     if list.list().len() == 0{
//         return;
//     }
//     let mut mut_defs : Vec<(&String, &MutListDef)> = Vec::new();
//     for (name,_, def) in def.default(){
//         match def{
//             ListDefValue::MilDef(d) =>{
//                 mut_defs.push((name, d));
//             },
//             _ =>{}
//         }
//     }
//     for (_,item) in list.list_mut().iter_mut(){
//         for &(name, def) in &mut_defs{
//             let has_list = item.values().contains_key(name);
//
//             if has_list {
//                 if let ListSabValue::Mil(Some(list)) = item.values_mut().get_mut(name).unwrap(){
//                     set_empty_mils(def.default(), list);
//                 }
//             } else{
//                 item.values_mut().insert(name.to_string(), ListSabValue::Mil(Some(MutListVal::crate_empty_list())));
//             }
//         }
//     }
// }