use docchi_core::structs::VarType;

pub(crate) fn into_qv(name : &str, var_type : VarType) -> String{
    if var_type == VarType::Normal {
        format!("Qv::Val({})", name)
    } else {
        format!("{}.into_qv()", name)
    }
}

