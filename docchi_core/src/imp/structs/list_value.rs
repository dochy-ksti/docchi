use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::rust_value::{RustValue, RustMemberType};
use crate::imp::structs::var_type::VarType;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::mut_list_def::MutListDef;
use crate::imp::structs::list_sab_value::ListSabValue;

#[derive(Debug, Clone, PartialEq)]
pub enum ListDefValue{
    Param(RustParam, VarType),
    //InnerDataDef(ListDefObj),
    CilDef(ListDefObj),
    MilDef(MutListDef),
}


impl ListDefValue{
    pub(crate) fn into_rust_value(self) -> RustValue{
        match self{
            ListDefValue::Param(p,v) => RustValue::Param(p,v),
            //ListDefValue::InnerDataDef(d) => RustValue::InnerDataDef(d),
            ListDefValue::CilDef(l) => RustValue::CilDef(l),
            ListDefValue::MilDef(m) => RustValue::MilDef(m),
        }
    }

    pub(crate) fn acceptable(&self, other : &ListSabValue) -> bool{
        if self.type_num() == other.type_num(){
            if self.value_type().acceptable(&other.qv_type()){
                return true;
            }
        }
        false
    }

    // pub(crate) fn compatible(&self, other : &ListDefValue) -> bool{
    //     if self.type_num() == other.type_num(){
    //         if self.value_type().compatible(&other.value_type()){
    //             return true;
    //         }
    //     }
    //     false
    // }


    pub(crate) fn value_type(&self) -> VarType {
        match self{
            ListDefValue::Param(_param, vt) => vt.clone(),
            ListDefValue::MilDef(obj) => if obj.undefiable() { VarType::Undefiable } else{ VarType::Normal }
            _ => VarType::Normal,
        }
    }

    pub(crate) fn type_num(&self) -> RustMemberType {
        use RustMemberType::*;
        match self{
            ListDefValue::Param(param, _) => param.type_num(),
            //ListDefValue::InnerDataDef(_) => InnerData,
            ListDefValue::CilDef(_) => Cil,
            ListDefValue::MilDef(_) => Mil,
        }
    }

    // pub(crate) fn inner_def(&self) -> Option<&ListDefObj>{
    //     match self{
    //         //ListDefValue::InnerDataDef(d) => Some(d),
    //         ListDefValue::CilDef(d) => Some(d),
    //         ListDefValue::MilDef(obj) => Some(obj.list_def()),
    //         _ => None,
    //     }
    // }
}

