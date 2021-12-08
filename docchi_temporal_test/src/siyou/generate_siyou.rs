#[cfg(test)]
mod tests {
    use docchi::core::json_dir_to_root;
    use docchi::intf::generate_interface;
    use anyhow::Result;

    //use crate::rust_to_json_new_default;
    //use crate::imp::json_to_rust::json_root_to_rust;
    //use crate::imp::rust_to_json::root_to_json::root_to_json_new_default;

    ///仕様から生成したコードがコンパイル通るかだけ調べる
    #[test]
    fn it_works() -> Result<()>{

        // match json_dir_to_rust("src/test/siyou", true) {
        match json_dir_to_root("../docchi_core/src/json_dir/json_siyou", true) {
            Ok(mut a) => {
                //println!("{:?}", a);
                let ans = generate_interface(&mut a);
                let source = ans.to_string();
                std::fs::write("src/siyou/siyou_generated.rs", &source)?;
            },
            Err(e) => { println!("val 1 {}", e.to_string()); assert!(false); }
        }
        Ok(())
    }
}