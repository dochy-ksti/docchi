use docchi_json5::{JVal, Span};
use super::names::Names;
use crate::error::CoreResult;
use super::json_array_to_rust::GatResult;
use crate::imp::structs::var_type::VarType;
use crate::imp::structs::rust_value::{RustValue};
use crate::imp::structs::qv::Qv;
use crate::imp::structs::rust_param::RustParam;

pub(crate) fn array_null_or_undefined(a : &[JVal], gat : GatResult, var_type: VarType, span : &Span, names : &Names) -> CoreResult<RustValue> {
    if a.len() != 1 {
        Err(format!(r#"{} {} null must be ["type", null] {}"#, span.line_str(), span.slice(), names))?
    }

    let val = match a[0] {
        JVal::Null(_) => {
            match gat {
                GatResult::Float => RustValue::Param(RustParam::Float(Qv::Null), var_type),
                GatResult::Int => RustValue::Param(RustParam::Int(Qv::Null), var_type),
                GatResult::Str => RustValue::Param(RustParam::String(Qv::Null), var_type),
                GatResult::Bool => RustValue::Param(RustParam::Bool(Qv::Null), var_type),
                _ => unreachable!(),
            }
        },
        JVal::Undefined(_) =>{
            match gat {
                GatResult::Float => RustValue::Param(RustParam::Float(Qv::Undefined), var_type),
                GatResult::Int => RustValue::Param(RustParam::Int(Qv::Undefined), var_type),
                GatResult::Str => RustValue::Param(RustParam::String(Qv::Undefined), var_type),
                GatResult::Bool => RustValue::Param(RustParam::Bool(Qv::Undefined), var_type),
                _ => unreachable!(),
            }
        },
        JVal::Int(d, _) =>{
            match gat{
                GatResult::Float => RustValue::Param(RustParam::Float(Qv::Val(d as f64)), var_type),
                _ => Err(format!(r#"{} {} null must be ["type", null] {}"#, span.line_str(), span.slice(), names))?,
            }
        },
        JVal::Double(d, _) =>{
            match gat{
                GatResult::Float => RustValue::Param(RustParam::Float(Qv::Val(d)), var_type),
                _ => Err(format!(r#"{} {} null must be ["type", null] {}"#, span.line_str(), span.slice(), names))?,
            }
        },
        _ =>{ Err(format!(r#"{} {} null must be ["type", null] {}"#, span.line_str(), span.slice(), names))? }
    };
    Ok(val)
}