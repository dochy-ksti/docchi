use crate::imp::common::path::hash::{hash_to_folder_name, folder_name_to_hash};

#[test]
fn test_hex_file_name() {
    let val: u128 = 123456789123456789123456789;
    let file_name = hash_to_folder_name(val);
    match folder_name_to_hash(&file_name) {
        Some(hex) => {
            assert_eq!(val, hex);
        }
        None =>{
            assert!(false)
        },
    }
}