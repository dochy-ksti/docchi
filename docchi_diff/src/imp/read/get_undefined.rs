use docchi_core::structs::{ParamType, RustParam, Qv};

pub(crate) fn get_undefined(pt : ParamType) -> RustParam{
    match pt {
        ParamType::Bool => { RustParam::Bool(Qv::Undefined) }
        ParamType::Int => { RustParam::Int(Qv::Undefined) }
        ParamType::Float => { RustParam::Float(Qv::Undefined) }
        ParamType::String => { RustParam::String(Qv::Undefined) }
        ParamType::IntArray => { RustParam::IntArray(Qv::Undefined) }
        ParamType::FloatArray => { RustParam::FloatArray(Qv::Undefined) }
        ParamType::Binary =>{ RustParam::Binary(Qv::Undefined) }
    }
}