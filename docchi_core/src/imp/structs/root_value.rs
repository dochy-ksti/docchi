use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::var_type::VarType;
use crate::imp::structs::rust_list::{ConstTable, ConstList};
use crate::imp::structs::rust_value::RustValue;
use crate::imp::structs::mut_list_def::MutListDef;
use crate::error::CoreResult;
use crate::imp::structs::root_sab_value::RootSabValue;

#[derive(Debug, Clone, PartialEq)]
pub enum RootValue{
    Param(RustParam, VarType),
    Table(ConstTable),
    CList(ConstList),
    MList(MutListDef),
}

impl RootValue{

    pub fn into_rust_value(self, sab : RootSabValue) -> CoreResult<RustValue>{
        match self{
            RootValue::Param(p,v) => Ok(RustValue::Param(p,v)),
            RootValue::Table(d) => Ok(RustValue::Table(d)),
            RootValue::CList(d) => Ok(RustValue::CList(d)),
            RootValue::MList(d) =>{
                if let RootSabValue::Mut(m) = sab {
                    Ok(RustValue::MList((d, m)))
                } else{
                    Err("unmatched Mut List")?
                }
            }
        }
    }
}

birudo dekinaku suruyo

