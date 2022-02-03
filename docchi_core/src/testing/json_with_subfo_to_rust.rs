#[cfg(test)]
mod tests {
    use crate::{root_to_json_new_default, json_dir_to_root};

    #[test]
    fn it_works() {
        match json_dir_to_root( "src/json_dir/subfol", true) {
            Ok(a) => {
                match root_to_json_new_default(&a){
                    Ok(_json) =>{
                        //println!("{}", json.to_string_pretty());
                    },
                    Err(e) =>{ println!("val 2 {}", e); }
                }
            },
            Err(e) => { println!("val 1 {}", e) }
        }
    }





}