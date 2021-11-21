use docchi_core::structs::{RustParam, MetaParam, VarType, QvType, RustIntArray, RustFloatArray, RustBinary};
use docchi_compaction::kval_enum::KVal;
use docchi_compaction::basic_compaction::{comp_int, comp_double, comp_str};
use crate::diff_error::{DiffError};

pub(crate) fn write_param(param : &RustParam, meta : &MetaParam, r : &mut Vec<KVal>) -> Result<(), DiffError>{
    match meta.var_type(){
        VarType::Normal =>{
            match param.qv_type() {
                QvType::Val => {
                    write_param2(param, r)
                },
                QvType::Null =>{
                    Err("invalid null")?
                },
                QvType::Undefined =>{
                    Err("invalid undefined")?
                }
            }
        },
        VarType::Nullable =>{
            match param.qv_type(){
                QvType::Val =>{
                    r.push(KVal::Bit(true));
                    write_param2(param, r)
                },
                QvType::Null =>{
                    r.push(KVal::Bit(false));
                    Ok(())
                },
                QvType::Undefined =>{ Err("invalid undefined")? }
            }
        },
        VarType::Undefiable =>{
            match param.qv_type(){
                QvType::Val =>{
                    r.push(KVal::Bit(true));
                    write_param2(param, r)
                },
                QvType::Undefined =>{
                    r.push(KVal::Bit(false));
                    Ok(())
                },
                QvType::Null =>{ Err("invalid null")? }
            }
        },
        VarType::UndefNullable =>{
            match param.qv_type(){
                QvType::Val =>{
                    r.push(KVal::Bit(true));
                    write_param2(param, r)
                },
                QvType::Null =>{
                    r.push(KVal::Bit(false));
                    r.push(KVal::Bit(true));
                    Ok(())
                },
                QvType::Undefined =>{
                    r.push(KVal::Bit(false));
                    r.push(KVal::Bit(false));
                    Ok(())
                },
            }
        }
    }
}

fn write_param2(param : &RustParam, r : &mut Vec<KVal>) -> Result<(), DiffError>{
    match param{
        RustParam::Bool(i) => r.push(KVal::Bit(*i.value().unwrap())),
        RustParam::Int(i) => r.push(comp_int(*i.value().unwrap())),
        RustParam::Float(f) => r.push(comp_double(*f.value().unwrap())),
        RustParam::String(s) => r.push( comp_str(s.value().unwrap().to_string())),
        RustParam::IntArray(s) => write_int_array(s.value().unwrap(), r),
        RustParam::FloatArray(s) => write_float_array(s.value().unwrap(), r),
        RustParam::Binary(s) => write_binary(s.value().unwrap(), r),
    }
    Ok(())
}

fn write_int_array(a : &RustIntArray, r : &mut Vec<KVal>){
    r.push(KVal::Binary8(a.vec().iter().map(|i| *i as u64).collect()))
}

fn write_float_array(a : &RustFloatArray, r : &mut Vec<KVal>){
    r.push(KVal::Binary8(a.vec().iter().map(|i| i.to_bits()).collect()))
}

fn write_binary(a : &RustBinary, r : &mut Vec<KVal>){
    //DocchiBenchの結果を見るに、Arcを使う最適化によって、誤差より多少マシ程度には早くなったと思う。でもそんなに価値はなさそう
    r.push(KVal::BinaryArc(a.arc().clone()));
}
