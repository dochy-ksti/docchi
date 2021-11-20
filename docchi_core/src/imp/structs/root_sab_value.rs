use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::rust_list::{ MutListVal};
use crate::IdentityEqual;

#[derive(Debug, Clone)]
pub enum RootSabValue{
    Param(RustParam),
    Mut(Option<MutListVal>),
}

impl RootSabValue{
    // pub(crate) fn type_num(&self) -> RustMemberType {
    //     use RustMemberType::*;
    //
    //     match self{
    //         RootSabValue::Param(param) => param.type_num(),
    //         RootSabValue::Mut(_) => Mil,
    //     }
    // }

    // pub(crate) fn into_rust_value_for_json(self) -> RustValue{
    //     match self{
    //         RootSabValue::Param(p) => RustValue::Param(p, VarType::Normal),
    //         RootSabValue::Mut(m) => RustValue::Mil(m),
    //     }
    // }

    // pub(crate) fn qv_type(&self) -> QvType{
    //     match self{
    //         RootSabValue::Param(p) => p.qv_type(),
    //         RootSabValue::Mut(m) => if m.is_some(){ QvType::Val } else{ QvType::Undefined },
    //     }
    // }
}

impl IdentityEqual for RootSabValue{
    fn identity_eq(&self, other: &Self) -> bool {
        match self{
            RootSabValue::Param(p) => if let RootSabValue::Param(p2) = other{ p.identity_eq(p2) } else{ false }
            RootSabValue::Mut(m) => if let RootSabValue::Mut(m2) = other{
                if let Some(m) = m{
                    if let Some(m2) = m2{
                        m.identity_eq(m2)
                    } else{ false }
                }
                else if m.is_none() && m2.is_none(){ true }
                else{ false }
            } else{ false },
        }
    }
}