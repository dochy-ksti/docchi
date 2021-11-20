// use crate::structs::{RootObject, RootValue, RootSabValue};
// use crate::imp::json_to_rust::set_empty_mils::set_empty_mils::set_empty_mils;
//
// pub(crate) fn set_empty_mils_root(root : &mut RootObject){
//
//     let (def, sabun, _) = root.def_and_mut_sab();
//     for (name, (_id, val)) in def.def(){
//
//         //RootはOldでも値を入れざるを得ないので入れて良い
//         //なのでここではOldは無視
//         match val {
//             RootValue::Param(_, _) => {},
//             RootValue::Table(_) =>{},
//             RootValue::CList(_) =>{}
//             RootValue::MList(def) =>{
//                 match sabun.get_mut(name){
//                     Some(RootSabValue::Mut(Some(l))) =>{
//                         set_empty_mils(def.default(), l);
//                     },
//                     _ => unreachable!(),
//                 }
//             },
//         }
//     }
// }