use docchi_core::intf::member_desc::MemberDesc;
use crate::imp::structs::param_source::ParamSource;
use docchi_core::structs::{RustMemberType, ParamType};
use crate::imp::structs::table_source::TableSource;
use crate::imp::structs::clist_source::CListSource;
use crate::imp::structs::cil_source::CilSource;
use crate::imp::structs::mil_source::MilSource;
use crate::imp::structs::mlist_source::MListSource;

#[derive(Debug, PartialEq)]
pub(crate) enum MemberSource{
    Param(ParamSource),
    Table(TableSource),
    CList(CListSource),
    MList(MListSource),
    Cil(CilSource),
    Mil(MilSource),
}

pub(crate) fn to_member_source(mem : &MemberDesc) -> MemberSource{
    match mem.member_type(){
        RustMemberType::Bool =>{
            MemberSource::Param(ParamSource::new(
                mem.name().to_string(),
                mem.var_type(),
                ParamType::Bool,
                mem.is_old(),
            ))
        },
        RustMemberType::Int =>{
            MemberSource::Param(ParamSource::new(
                mem.name().to_string(),
                mem.var_type(),
                ParamType::Int,
                mem.is_old(),
            ))
        },
        RustMemberType::Float =>{
            MemberSource::Param(ParamSource::new(
                mem.name().to_string(),
                mem.var_type(),
                ParamType::Float,
                mem.is_old(),
            ))
        },
        RustMemberType::Str =>{
            MemberSource::Param(ParamSource::new(
                mem.name().to_string(),
                mem.var_type(),
                ParamType::String,
                mem.is_old(),
            ))
        },
        RustMemberType::IntArray =>{
            MemberSource::Param(ParamSource::new(
                mem.name().to_string(),
                mem.var_type(),
                ParamType::IntArray,
                mem.is_old(),
            ))
        },
        RustMemberType::FloatArray =>{
            MemberSource::Param(ParamSource::new(
                mem.name().to_string(),
                mem.var_type(),
                ParamType::FloatArray,
                mem.is_old(),
            ))
        },
        RustMemberType::Binary =>{
            MemberSource::Param(ParamSource::new(
                mem.name().to_string(),
                mem.var_type(),
                ParamType::Binary,
                mem.is_old(),
            ))
        },
        RustMemberType::Table =>{
            MemberSource::Table(TableSource::from(mem))
        }
        RustMemberType::CList =>{
            MemberSource::CList(CListSource::from(mem))
        }
        RustMemberType::Cil =>{
            MemberSource::Cil(CilSource::from(mem))
        }
        RustMemberType::MList =>{
            MemberSource::MList(MListSource::from(mem))
        },
        RustMemberType::Mil =>{
            MemberSource::Mil(MilSource::from(mem))
        },
        //_ => unreachable!(),
    }
}