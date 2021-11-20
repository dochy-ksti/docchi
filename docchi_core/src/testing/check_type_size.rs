// #[cfg(test)]
// mod tests {
//     use crate::imp::structs::rust_value::RustValue;
//     use crate::imp::structs::root_value::RootValue;
//     use crate::imp::structs::list_value::{ListDefValue};
//     use crate::imp::structs::ref_value::{RefValue, RefSabValue};
//     use crate::imp::structs::rust_param::RustParam;
//     use crate::imp::structs::rust_list::{ConstList};
//     use crate::imp::structs::list_def_obj::ListDefObj;
//     use crate::structs::{ListSabValue, RootSabValue, MutItem};
//
//     #[test]
//     fn check_size() {
//         println!("RustValue {} ", std::mem::size_of::<RustValue>());
//         println!("RootValue {} ", std::mem::size_of::<RootValue>());
//         //問題はSabValueのRustParamをBoxに入れるかだろうなあ でもParam一個32バイトで何か問題あるか？
//         println!("RootSabValue {} ", std::mem::size_of::<RootSabValue>());
//         println!("ListDefValue {} ", std::mem::size_of::<ListDefValue>());
//         println!("ListSabValue {} ", std::mem::size_of::<ListSabValue>());
//         println!("refvalue {} ", std::mem::size_of::<RefValue>());
//         println!("RefSabValue {} ", std::mem::size_of::<RefSabValue>());
//         println!("RustParam {} ", std::mem::size_of::<RustParam>());
//         println!("ConstList {} ", std::mem::size_of::<ConstList>());
//         println!("ListDefObj {} ", std::mem::size_of::<ListDefObj>());
//         println!("MutItem {} ", std::mem::size_of::<MutItem>());
//     }
// }