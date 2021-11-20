#[cfg(test)]
mod tests {
    use crate::{root_to_json_new_default, json_dir_to_root};
    use crate::imp::json_to_rust::json_root_to_rust;

    #[test]
    fn it_works() {
        match json_dir_to_root( "src/json_dir/json_siyou", true) {
            Ok(a) => {
                match root_to_json_new_default(&a){
                    Ok(_json) =>{
                        //println!("{}", _json.to_string_pretty());
                    },
                    Err(e) =>{ println!("val 2 {}", e); }
                }
            },
            Err(e) => { println!("val 1 {}", e) }
        }
    }




    #[test]
    fn it_works2() {
        match json_dir_to_root("src/json_dir/json_siyou", true){
            Ok(a) => {
                match root_to_json_new_default(&a) {
                    Ok(a_v) => {
                        let av_s = a_v.to_string_pretty();
                        match json_root_to_rust(&av_s){
                            Ok(b) =>{
                                match root_to_json_new_default(&b){
                                    Ok(b_v) =>{
                                        let bv_s = b_v.to_string_pretty();

                                        match json_root_to_rust(&bv_s){
                                            Ok(c) =>{
                                                match root_to_json_new_default(&c){
                                                    Ok(c_v) =>{
                                                        let cv_s = c_v.to_string_pretty();
                                                        assert_eq!(bv_s, cv_s);
                                                        //println!("{}", cv_s);
                                                    },
                                                    Err(e) =>println!("type6 {}", e),
                                                }

                                            },
                                            Err(e) =>println!("type5 {}", e),
                                        }
                                    },
                                    Err(e) => println!("type4 {}", e),
                                }
                            }
                            Err(e) => println!("type3 {}", e)
                        }
                    },
                    Err(e) => { println!("type2 {}", e); }
                }
            },
            Err(e) => println!("type1 {}", e),
        }
    }
//

}