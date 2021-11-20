use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::var_type::VarType;

pub(crate ) fn verify_set_sabun(p : &RustParam, vt : &VarType, sab : &RustParam) -> Result<(), SetSabunError>{
    if p.acceptable(sab) == false{
        return Err(SetSabunError::ParamTypeMismatch);
    }
    if vt.acceptable( &sab.qv_type()) == false{
        return Err(SetSabunError::QvTypeMismatch);
    }
    return Ok(());
}

pub enum SetSabunError{
    ParamNotFound, ParamTypeMismatch, QvTypeMismatch
}