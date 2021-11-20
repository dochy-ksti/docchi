use crate::imp::rust_to_json::rust_array_to_json::rust_array_to_json;
use crate::imp::structs::qv::Qv;
use crate::imp::structs::my_json::Value;
use crate::imp::structs::rust_param::RustParam;

pub(crate) fn get_param(v : &RustParam) -> Value{
    let r = match v{
        RustParam::Bool(b) => to(b,  "Bool",|b| Value::Bool(*b)),
        RustParam::String(s) => to(s, "Str", |s| Value::String(s.str().to_string())),
        RustParam::Float(n)=> to(n, "Float",
                                 |n| Value::Array(vec![Value::String("Float".to_string()), Value::Number(*n)])),
        RustParam::Int(n)=> to(n, "Int", |n| Value::Number(*n as f64)),
        _ =>{
            let (array, at) = v.to_rust_array().unwrap();
            rust_array_to_json(&array, &at)
        },
    };
    return r;
}

fn to<T : Clone>(qv : &Qv<T>, type_name : &str, f : impl Fn(&T)->Value) -> Value{
    match qv{
        Qv::Val(v) => f(v),
        Qv::Null => Value::Array(vec![Value::String(type_name.to_string()), Value::Null]),
        Qv::Undefined => Value::Array(vec![Value::String(type_name.to_string()), Value::Undefined]),
    }
}