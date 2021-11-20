use docchi_core::structs::{ParamType, RustParam, Qv};

pub(crate) fn get_null(pt : ParamType) -> RustParam{
    match pt {
        ParamType::Bool => { RustParam::Bool(Qv::Null) }
        ParamType::Int => { RustParam::Int(Qv::Null) }
        ParamType::Float => { RustParam::Float(Qv::Null) }
        ParamType::String => { RustParam::String(Qv::Null) }
        ParamType::IntArray => { RustParam::IntArray(Qv::Null) }
        ParamType::FloatArray => { RustParam::FloatArray(Qv::Null) }
        ParamType::Binary => { RustParam::Binary(Qv::Null)}
    }
}