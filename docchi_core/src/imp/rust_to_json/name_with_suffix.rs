use crate::imp::structs::var_type::VarType;

pub(crate) fn name_with_suffix(name : &str, vt : VarType) -> String{
    format!("{}{}", name, vt.to_suffix())
}