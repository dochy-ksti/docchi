//use super::json_list_to_rust::json_list_to_rust;
use crate::error::CoreResult;
use super::names::Names;
use docchi_json5::{JVal, Span};
use super::list::json_list_to_rust::json_list_to_rust;
use crate::imp::json_to_rust::array_null::array_null_or_undefined;
use crate::imp::structs::var_type::VarType;
use crate::imp::structs::rust_value::{RustValue};
use crate::imp::structs::array_type::ArrayType;
use crate::imp::structs::qv::Qv;
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::rust_array::RustArray;

pub(crate) fn json_array_to_rust(array : &Vec<JVal>, var_type: VarType, span : &Span, names : &Names) -> CoreResult<RustValue>{
    use GatResult::*;
    let gat = get_array_type(array);
    return match gat{
        AT(array_type) => {
            let array = get_array(&array[1..], &array_type, names)?;
            Ok(RustValue::Param(array.to_param(&array_type).or_else(|e| {
                Err(format!(r#"{} {}: {}"#, span.line_str(), names, e.to_string()))
            })?, var_type))
        },
        NoTagInt =>{
            let array = get_array(&array, &ArrayType::Int, names)?;
            Ok(RustValue::Param(array.to_param(&ArrayType::Int).unwrap(), var_type))
        }
        Float | Int | Str | Bool =>{
            array_null_or_undefined(&array[1..], gat, var_type, span, names)
        },
        InnerMutUndefined =>{
            Ok(RustValue::Mil(None))
        },
        NotDefined =>{ Err(format!(r#"{} Array must be "...Array", "Clist", "Table", "MList", "Cil", "Mil", "CilDef", "MilDef", "Int", "Float", "Str" or "Bool" {}"#, span.line_str(), names))? },
        CList | Table | MList | Cil | //InnerData |
        Mil | CilDef | //InnerDataDef |
        MilDef  =>{
            match var_type {
                VarType::Normal =>{
                    let tmp = json_list_to_rust(&array[1..], span, names)?;
                    match gat{
                        CList => Ok(RustValue::CList(tmp.into_const_list()?)),
                        Table => Ok(RustValue::Table(tmp.into_const_data()?)),
                        MList => Ok(RustValue::MList(tmp.into_mut_list(false)?)),
                        Cil => Ok(RustValue::Cil(tmp.into_inner_list()?)),
                        Mil => Ok(RustValue::Mil(Some(tmp.into_mut_inner_list()?))),
                        CilDef => Ok(RustValue::CilDef(tmp.into_inner_def()?)),
                        MilDef => Ok(RustValue::MilDef(tmp.into_inner_mut_def(false)?)),
                        _ => unreachable!() ,
                    }
                },
                VarType::Undefiable =>{
                    let tmp = json_list_to_rust(&array[1..], span, names)?;
                    match gat {
                        MilDef => Ok(RustValue::MilDef(tmp.into_inner_mut_def(true)?)),
                        MList => Ok(RustValue::MList(tmp.into_mut_list(true)?)),
                        _ =>{
                            Err(format!(r#"{} Lists can't be undefined {} except for InnerMut"#, span.line_str(), names))?
                        }
                    }
                }
                _ =>{
                    Err(format!(r#"{} Lists can't be null {}"#, span.line_str(), names))?
                }
            }
        },
    }
}

pub(crate) enum GatResult{
    AT(ArrayType),
    CList,
    Table,
    MList,
    Cil,
    Mil,
    CilDef,
    MilDef,
    Int,
    NoTagInt,
    Float,
    Str,
    Bool,
    InnerMutUndefined,
    NotDefined,
}

fn get_array_type(a : &Vec<JVal>) -> GatResult{
    use GatResult::*;
    if let Some(v) = a.get(0){
        return match v{
            JVal::String(s, _)=>{
                return match s.as_str(){
                    "IntArray" =>{ AT(ArrayType::Int) },
                    "FloatArray" =>{ AT(ArrayType::Float) },
                    "Binary" =>{ AT(ArrayType::Binary) },
                    "Int" =>{ Int },
                    "Float" =>{ Float },
                    "Str" =>{ Str },
                    "Bool" =>{ Bool },
                    "CList" => { GatResult::CList },
                    "Table" => { GatResult::Table },
                    "MList" => { GatResult::MList },
                    "Cil" => { GatResult::Cil },
                    "Mil" =>{ GatResult::Mil },
                    "CilDef" => { GatResult::CilDef },
                    "MilDef" =>{ GatResult::MilDef },
                    "__InnerMutUndefined" => { GatResult::InnerMutUndefined },
                    _=>{ GatResult::NotDefined },
                }
            },
            JVal::Int(_num, _) => NoTagInt,
            _ => NotDefined,
        }
    }
    NotDefined
}

pub(crate ) fn get_array(a : &[JVal], array_type : &ArrayType, names : &Names) -> CoreResult<RustArray>{
    let mut vec : Vec<RustParam> = Vec::with_capacity(a.len());
    for item in a{
        let val = match item{
            JVal::Double(f, _) => {
                match array_type {
                    ArrayType::Float => RustParam::Float(Qv::Val(*f)),
                    _ => Err(format!(r#"{} {} float is not valid in this array {}"#, item.line_str(), item.slice(), names))?,
                }
            },
            JVal::Int(i, _) => {
                match array_type {
                    ArrayType::Int | ArrayType::Binary => RustParam::Int(Qv::Val(*i)),
                    ArrayType::Float => RustParam::Float(Qv::Val(*i as f64)),
                    //_ => Err(format!(r#"{} {} int is not valid in this array {}"#, item.line_str(), item.slice(), names))?,
                }
            },
            JVal::String(_s, _) =>{
                match array_type {
                    //ArrayType::String => RustParam::String(Qv::Val(RustString::new(s.to_string()))),
                    _ => Err(format!(r#"{} {} string is not valid in this array {}"#, item.line_str(), item.slice(), names))?,
                }
            },
            JVal::Null(_) =>{
                if vec.len() == 0 && a.len() == 1{
                    return Ok(RustArray::new(Qv::Null));
                } else{
                    Err(format!(r#"{} null must be ["type", null] {}"#, item.line_str(), names))?
                }
            },
            JVal::Undefined(_) =>{
                if vec.len() == 0 && a.len() == 1{
                    return Ok(RustArray::new(Qv::Undefined));
                } else{
                    Err(format!(r#"{} undefined must be ["type", undefined] {}"#, item.line_str(), names))?
                }
            },
            JVal::Array(a2, span) =>{
                match array_type{
                    ArrayType::Float => {
                        let rv = json_array_to_rust(a2, VarType::Normal, span, names)?;
                        match rv{
                            RustValue::Param(RustParam::Float(f), _vt) =>{
                                match &f {
                                    Qv::Val(val) => RustParam::Float(Qv::Val(*val)),
                                    Qv::Null => {
                                        Err(format!(r#"{} null is not valid {}"#, item.line_str(), names))?
                                    },
                                    Qv::Undefined => {
                                        Err(format!(r#"{} undefined is not valid {}"#, item.line_str(), names))?
                                    },
                                }
                            },
                            _ =>{ Err(format!(r#"{} {} is not a float value {}"#, span.line_str(), span.slice(), names))? }
                        }
                    },
                    _ => Err(format!(r#"{} two-dimensional array is not valid {}"#, item.line_str(), names))?,
                }
            },
            JVal::Map(_,_) => Err(format!(r#"{} object is not valid for an array item {}"#, item.line_str(), names))?,
            JVal::Bool(_, _) => Err(format!(r#"{} bool is not valid for an array item {}"#, item.line_str(), names))?,
        };
        vec.push(val);
    }
    return Ok(RustArray::new(Qv::Val(vec)));
}