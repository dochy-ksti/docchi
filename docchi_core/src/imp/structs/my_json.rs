use std::collections::{BTreeMap};

#[derive(Debug)]
pub enum Value{
    Bool(bool),
    String(String),
    Number(f64),
    Array(Vec<Value>),
    ///HashMapだと毎回順序が変わるみたいなので、一応alphabetical orderにしておこう
    Map(BTreeMap<String, Value>),
    Null,
    Undefined,
}

impl Value{
    pub fn to_string_pretty(&self) -> String{
        let mut s = String::new();
        write(self, &mut s, 0);
        return s;
    }
}

fn write(value : &Value, s : &mut String, indent_level : usize){
    match value{
        Value::Bool(b) => s.push_str(&b.to_string()),
        Value::String(val) => s.push_str(&format!(r#""{}""#, val)),
        Value::Null => s.push_str("null"),
        Value::Undefined => s.push_str("undefined"),
        Value::Number(f) => s.push_str(&f.to_string()),
        Value::Array(v) => write_array(v, s, indent_level + 1),
        Value::Map(obj) => write_map(obj, s, indent_level + 1),
    }
}

fn write_array(array : &Vec<Value>, s : &mut String, indent_level : usize){
    if array.len() == 0{
        s.push_str("[]");
    } else if array.len() == 1 {
        s.push_str("[ ");
        write(&array[0], s, indent_level);
        s.push_str(" ]");
    } else if array.len() == 2 {
        s.push_str("[ ");
        write(&array[0], s, indent_level);
        s.push_str(", ");
        write(&array[1], s, indent_level);
        s.push_str(" ]");
    } else{
        s.push_str("[\n");
        s.push_str(&indent_str(indent_level));
        write(&array[0], s, indent_level);
        s.push_str(",\n");
        for item in &array[1..array.len()-1] {
            s.push_str(&indent_str(indent_level));
            write(item, s, indent_level);
            s.push_str(",\n");
        }
        s.push_str(&indent_str(indent_level));
        write(&array[array.len()-1], s, indent_level);
        s.push('\n');
        if indent_level != 0 {
            s.push_str(&indent_str(indent_level - 1));
        }
        s.push(']');
    }
}

fn indent_str(indent_level : usize) -> String{
    "  ".repeat(indent_level)
}

fn write_map(map : &BTreeMap<String, Value>, s : &mut String, indent_level : usize) {
    if map.len() == 0{
        s.push_str("{}");
    } else if map.len() == 1 {
        for (k,v) in map{
            s.push_str(&format!("{} \"{}\" : ","{", k));
            write(v, s, indent_level);
            s.push_str(" }");
        }
    } else{
        s.push_str("{\n");
        for (k,v) in map{
            s.push_str(&indent_str(indent_level));
            s.push_str(&format!("\"{}\" : ", k));
            write(v, s, indent_level);
            s.push_str(",\n");
        }
        if indent_level != 0 {
            s.push_str(&indent_str(indent_level - 1));
        }
        s.push_str("}");

    }
}