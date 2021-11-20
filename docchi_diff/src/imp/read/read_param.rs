use docchi_core::structs::{MetaParam, VarType, RustParam, ParamType, Qv, RustString, RustIntArray, RustFloatArray, RustBinary };
use crate::diff_error::{DiffError, OptToErr};
use crate::imp::read::reader::Reader;
use crate::imp::read::get_null::get_null;
use crate::imp::read::get_undefined::get_undefined;
use docchi_compaction::kval_enum::KVal;

pub(crate) fn read_param(meta : &MetaParam, r : &mut Reader) -> Result<RustParam, DiffError>{
    match meta.var_type() {
        VarType::Normal => {
            read_param2(meta, r)
        },
        VarType::Nullable => {
            if r.read()?.ast_bool()?{
                read_param2(meta, r)
            } else{
                Ok(get_null(meta.param_type()))
            }
        },
        VarType::Undefiable =>{
            if r.read()?.ast_bool()?{
                read_param2(meta, r)
            } else{
                Ok(get_undefined(meta.param_type()))
            }
        },
        VarType::UndefNullable =>{
            if r.read()?.ast_bool()?{
                read_param2(meta, r)
            } else if r.read()?.ast_bool()?{
                Ok(get_null(meta.param_type()))
            } else{
                Ok(get_undefined(meta.param_type()))
            }
        }
    }
}

fn read_param2(meta : &MetaParam, r : &mut Reader) -> Result<RustParam, DiffError> {
    let p = match meta.param_type() {
        ParamType::Bool => { RustParam::Bool(Qv::Val(r.read()?.ast_bool()?)) }
        ParamType::Int => { RustParam::Int(Qv::Val(r.read()?.ast_i64()?)) }
        ParamType::Float => { RustParam::Float(Qv::Val(r.read()?.ast_f64()?)) }
        ParamType::String => {
            RustParam::String(
                Qv::Val(RustString::new(r.read()?.ast_string()?)))
        }
        ParamType::IntArray => { RustParam::IntArray(Qv::Val(read_int_array(r)?)) }
        ParamType::FloatArray => { RustParam::FloatArray(Qv::Val(read_float_array(r)?)) }
        ParamType::Binary => { RustParam::Binary(Qv::Val(read_binary(r)?)) }
    };
    Ok(p)
}

fn read_int_array(r : &mut Reader) -> Result<RustIntArray, DiffError>{
    if let KVal::Binary8(b) = r.read()?{
        let (ptr, len, cap) = b.into_raw_parts();
        Ok(RustIntArray::new(unsafe{
            Vec::from_raw_parts(ptr as *mut i64, len, cap)
        }))
    } else{
        Err("IntArray couldn't be read")?
    }
}

fn read_float_array(r : &mut Reader) -> Result<RustFloatArray, DiffError>{
    if let KVal::Binary8(b) = r.read()?{
        let (ptr, len, cap) = b.into_raw_parts();
        Ok(RustFloatArray::new(unsafe{
            Vec::from_raw_parts(ptr as *mut f64, len, cap)
        }))
    } else{
        Err("FloatArray couldn't be read")?
    }
}


fn read_binary(r : &mut Reader) -> Result<RustBinary, DiffError>{
    if let KVal::Binary(b) = r.read()?{
        Ok(RustBinary::new(b))
    } else{
        Err("Binary couldn't be read")?
    }
}