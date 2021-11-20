use docchi_core::structs::{VarType};

pub(crate) fn with_var(t : &str, vt : VarType) -> String{
    match vt{
        VarType::Normal => t.to_string(),
        VarType::Nullable => format!("NullOr<{}>", t),
        VarType::Undefiable => format!("UndefOr<{}>", t),
        VarType::UndefNullable => format!("Qv<{}>", t),
    }
}

