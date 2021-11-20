use std::collections::BTreeMap;
use crate::imp::structs::my_json::Value;


#[derive(Debug)]
pub struct JsonFileImpl{
    pub filename_without_ext : String,
    pub json : String,
}

impl JsonFile for &JsonFileImpl{
    fn filename_without_ext(&self) -> &str {
        &self.filename_without_ext
    }

    fn json(&self) -> &str {
        &self.json
    }
}

pub trait JsonFile{
    fn filename_without_ext(&self) -> &str;
    fn json(&self) -> &str;
}

/// (filename_without_ext, json)
impl JsonFile for (&str, &str){
    fn filename_without_ext(&self) -> &str {
        self.0
    }

    fn json(&self) -> &str {
        self.1
    }
}

#[derive(Debug)]
pub struct JsonDir(pub BTreeMap<String, Value>);

impl JsonDir{
    pub fn to_string(&self) -> String{
        let mut result = String::new();
        let map = &self.0;
        for (name, value) in map{

            result.push_str(&format!("{} : {}\n",name, value.to_string_pretty()));
        }
        return result;
    }
}
