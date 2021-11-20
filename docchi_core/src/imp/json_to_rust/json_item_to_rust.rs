use super::Names;
use docchi_json5::JVal;
use crate::error::CoreResult;
use super::json_array_to_rust::json_array_to_rust;
use crate::imp::structs::var_type::VarType;
use crate::imp::structs::rust_value::{RustValue};
use crate::imp::structs::qv::Qv;
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::rust_string::RustString;


pub(crate) fn json_item_to_rust(name : &str, var_type: VarType, v : &JVal, names : &Names) -> CoreResult<RustValue> {
    let names = &names.append(name);
    match v {
        JVal::Bool(b, _) => {
            Ok(RustValue::Param(RustParam::Bool(Qv::Val(*b)), var_type))
        },
        JVal::Double(f, _)=>{
            Ok(RustValue::Param(RustParam::Float(Qv::Val(*f)), var_type))
        },
        JVal::Int(i, _) =>{
            Ok(RustValue::Param(RustParam::Int(Qv::Val(*i)), var_type))
        },
        JVal::String(s, _) => {
            let s = s.to_string();
            Ok(RustValue::Param(RustParam::String(Qv::Val(RustString::new(s))), var_type))
        },
        JVal::Array(a, _) => {
            Ok(json_array_to_rust(a, var_type, v.span(), names)?)
        },
        JVal::Map(_map, span) => {
            Err(format!("{} Objects can't have objects {}", span.line_str(), names))?
        },
        JVal::Null(span) =>{
            Err(format!(r#"{} null must be ["type", null] {}"#, span.line_str(), names))?
        },
        JVal::Undefined(span) =>{
            Err(format!(r#"{} undefined must be ["type", undefined] {}"#, span.line_str(), names))?
        }
    }
}

pub(crate) fn json_item_to_rust_ref(name : &str, value_type : VarType, v : &JVal, names : &Names) -> CoreResult<RustValue> {
    let names = &names.append(name);
    match v {
        JVal::Bool(_, span) => {
            Err(format!("{} {} Ref object's members must be string or null {}", span.line_str(), span.slice(), names))?
        },
        JVal::Double(_, span)=>{
            Err(format!("{} {} Ref object's members must be string or null {}", span.line_str(), span.slice(), names))?
        },
        JVal::Int(_, span)=>{
            Err(format!("{} {} Ref object's members must be string or null {}", span.line_str(), span.slice(), names))?
        },
        JVal::String(s, _) => {
            let s = s.to_string();
            Ok(RustValue::Param(RustParam::String(Qv::Val(RustString::new(s))), value_type))
        },
        JVal::Array(_, span) => {
            Err(format!("{} {} Ref object's members must be string or null {}", span.line_str(), span.slice(), names))?
        },
        JVal::Map(_, span) => {
            Err(format!("{} {} Ref object's members must be string or null {}", span.line_str(), span.slice(), names))?
        },
        JVal::Null(_) =>{
            Ok(RustValue::Param(RustParam::String(Qv::Null), value_type))
        },
        JVal::Undefined(_)=>{
            Ok(RustValue::Param(RustParam::String(Qv::Undefined), value_type))
        }
    }
}

