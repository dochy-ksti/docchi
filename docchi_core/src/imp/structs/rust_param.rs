use crate::imp::structs::qv::{Qv, QvType};
use crate::imp::structs::rust_string::{RustString };
use crate::imp::structs::rust_value::RustMemberType;
use crate::imp::structs::rust_array::{RustArray, RustIntArray, RustFloatArray, RustBinary};
use crate::imp::structs::array_type::ArrayType;
use crate::imp::structs::util::identity_equal_trait::IdentityEqual;

#[derive(Debug,  Clone, PartialEq)]
pub enum RustParam{
    Bool(Qv<bool>),
    Float(Qv<f64>),
    Int(Qv<i64>),
    String(Qv<RustString>),
    IntArray(Qv<RustIntArray>),
    FloatArray(Qv<RustFloatArray>),
    Binary(Qv<RustBinary>),
    //StrArray(Qv<RustStrArray>),
    //Num2Array(Qv<RustNum2Array>)
}


impl RustParam {
    pub fn qv_type(&self) -> QvType {
        match self {
            RustParam::Bool(b) => b.qv_type(),
            RustParam::Float(f) => f.qv_type(),
            RustParam::Int(i) => i.qv_type(),
            RustParam::String(s) => s.qv_type(),
            RustParam::IntArray(a) => a.qv_type(),
            RustParam::FloatArray(a) => a.qv_type(),
            RustParam::Binary(a) => a.qv_type()
            //RustParam::StrArray(a) => a.qv_type(),
            //RustParam::Num2Array(a) => a.qv_type(),
        }
    }

    pub(crate) fn type_num(&self) -> RustMemberType {
        use RustMemberType::*;
        match self {
            RustParam::Bool(_) => Bool,
            RustParam::Float(_) => Float,
            RustParam::Int(_) => Int,
            RustParam::String(_) => Str,
            RustParam::FloatArray(_) => FloatArray,
            RustParam::IntArray(_) => IntArray,
            RustParam::Binary(_) => Binary,
            //RustParam::StrArray(_) => StrArray,
            //RustParam::Num2Array(_) => Num2Array,
        }
    }


    pub(crate) fn acceptable(&self, other: &Self) -> bool {
        if self.type_num() != other.type_num() {
            return false;
        }
        return true;
    }

    ///型情報を維持したままundefinedに変える
    pub(crate) fn to_undefined(&self) -> Self {
        match self {
            RustParam::Bool(_) => RustParam::Bool(Qv::Undefined),
            RustParam::Float(_) => RustParam::Float(Qv::Undefined),
            RustParam::Int(_) => RustParam::Int(Qv::Undefined),
            RustParam::String(_) => RustParam::String(Qv::Undefined),
            RustParam::FloatArray(_) => RustParam::FloatArray(Qv::Undefined),
            RustParam::IntArray(_) => RustParam::IntArray(Qv::Undefined),
            RustParam::Binary(_) => RustParam::Binary(Qv::Undefined)
            //RustParam::StrArray(_) => RustParam::StrArray(Qv::Undefined),
            //RustParam::Num2Array(_) => RustParam::Num2Array(Qv::Undefined)
        }
    }

    pub(crate) fn to_rust_array(&self) -> Option<(RustArray, ArrayType)>{
        match self{
            RustParam::FloatArray(a) => Some((RustArray::from_float_array(a), ArrayType::Float)),
            RustParam::IntArray(a) => Some((RustArray::from_int_array(a), ArrayType::Int)),
            RustParam::Binary(a) => Some((RustArray::from_binary(a), ArrayType::Binary)),
            //RustParam::StrArray(a) => Some((RustArray::from_str_array(a), ArrayType::String)),
            //RustParam::Num2Array(a) => Some((RustArray::from_num2_array(a), ArrayType::Num2)),
            _ => None,
        }
    }

    pub fn to_default_value(&self, len : usize) -> RustParam{
        match self{
            RustParam::Bool(_) => RustParam::Bool(Qv::Val(false)),
            RustParam::Float(_) => RustParam::Float(Qv::Val(0.0)),
            RustParam::Int(_) => RustParam::Int(Qv::Val(0)),
            RustParam::String(_) => RustParam::String(Qv::Val(RustString::new("".to_string()))),
            RustParam::FloatArray(_) => RustParam::FloatArray(Qv::Val(RustFloatArray::new(vec![0.0; len]))),
            RustParam::IntArray(_) => RustParam::IntArray(Qv::Val(RustIntArray::new(vec![0; len]))),
            RustParam::Binary(_) => RustParam::Binary(Qv::Val(RustBinary::new(vec![0; len])))
        }
    }

    pub(crate) fn to_float(&self) -> Option<f64>{
        if let RustParam::Float(Qv::Val(s)) = self { Some(*s) } else{ None }
    }

    pub(crate) fn to_int(&self) -> Option<i64>{
        if let RustParam::Int(Qv::Val(s)) = self { Some(*s) } else{ None }
    }

    pub(crate) fn to_u8(&self) -> Option<u8>{
        if let RustParam::Int(Qv::Val(s)) = self{
            if (*s as u8) as i64 == *s{
                return Some(*s as u8)
            }
        }
        return None;
    }
}

impl IdentityEqual for RustParam{
    fn identity_eq(&self, other: &Self) -> bool {
        match self{
            RustParam::Bool(s) => if let RustParam::Bool(o) = other{ s.identity_eq(o) } else { false }
            RustParam::Int(s) => if let RustParam::Int(o) = other{ s.identity_eq(o) } else { false }
            RustParam::Float(s) => if let RustParam::Float(o) = other{ s.identity_eq(o) } else { false }
            RustParam::String(s) => if let RustParam::String(o) = other{ s.identity_eq(o) } else { false }
            RustParam::IntArray(s) => if let RustParam::IntArray(o) = other{ s.identity_eq(o) } else { false }
            RustParam::FloatArray(s) => if let RustParam::FloatArray(o) = other{ s.identity_eq(o) } else { false }
            RustParam::Binary(s) => if let RustParam::Binary(o) = other{ s.identity_eq(o) } else { false }


        }
    }
}