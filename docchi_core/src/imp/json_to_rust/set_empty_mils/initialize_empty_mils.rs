// use crate::structs::{ListDefObj, MutListVal, ListDefValue};
// use crate::imp::structs::list_sab_value::ListSabValue;
// use crate::{HashMt, HashM};
//
// pub(crate) fn initialize_empty_mils(def : &ListDefObj) -> HashM<String, ListSabValue>{
//     let mut hash : HashM<String, ListSabValue> = HashMt::new();
//     for (name,_, def) in def.default(){
//         match def{
//             ListDefValue::MilDef(_d) =>{
//                 hash.insert(name.to_string(), ListSabValue::Mil(Some(MutListVal::crate_empty_list())));
//             },
//             _ =>{}
//         }
//     }
//     hash
// }