use docchi_json5::{JVal};
use crate::imp::json_to_rust::names::Names;
use crate::error::CoreResult;
use crate::imp::json_to_rust::json_name::{json_name, NameType};
use crate::imp::structs::var_type::VarType;
use crate::imp::structs::util::hash_m::{HashS, HashSt};

pub(crate) fn get_old(array : &[JVal], names : &Names) -> CoreResult<HashS<String>>{
    let mut result : HashS<String> = HashSt::with_capacity(array.len());

    for item in array{
        match item{
            JVal::String(s, span) =>{
                match json_name(s){
                    Some(NameType::Name(name, VarType::Normal)) =>{
                        result.insert(name);
                    },
                    _ =>{
                        Err(format!("{} {} is not a valid simple name {}",span.line_str(), s, names))?;
                    }
                }
            },
            _ =>{
                let span = item.span();
                Err(format!(r#"{} {} old must be strings {}"#, span.line_str(), span.slice(), names))?
            }
        }
    }
    return Ok(result);
}