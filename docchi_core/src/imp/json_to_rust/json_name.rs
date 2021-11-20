use regex::Regex;
use regex::Captures;
//use lazy_static::lazy_static;
use crate::imp::structs::var_type::VarType;
use once_cell::sync::Lazy;

pub(crate) enum SystemNames{
    Old,
    ID,
    Ref,
    Enum,
}

pub(crate) enum NameType{
    Name(String, VarType),
    SystemName(SystemNames)
}

pub(crate) fn json_name(s : &str) -> Option<NameType>{
    fn some(sn: SystemNames) -> Option<NameType> {
        return Some(NameType::SystemName(sn));
    }
    use SystemNames::*;

    match s {
        "Old" => return some(Old),
        "ID" => return some(ID),
        "Ref" => return some(Ref),
        "Enum" => return some(Enum),
        _ => {},
    }

    let (vt, name) = value_type_and_name(s)?;
    return Some(NameType::Name(name, vt))
}

///?とか！がついておらず大文字で始まらない普通の名前
pub(crate) fn json_simple_name(s : &str) -> Option<String> {
    match json_name(s) {
        Some(NameType::Name(name, VarType::Normal)) => {
            Some(name)
        },
        _ => { None }
    }
}

// pub(crate ) fn dot_chained_name(s : &str) -> Option<Vec<&str>>{
//     let splitted : Vec<&str> = s.split('.').collect();
//     for item in &splitted{
//         if json_simple_name(item).is_none(){
//             return None;
//         }
//     }
//     return Some(splitted)
// }
//
// pub(crate ) fn is_dot_chained_name(s : &str) -> bool{
//     for item in s.split('.'){
//         if json_simple_name(item).is_none(){
//             return false;
//         }
//     }
//     return true
// }


// ///[@a-z_][a-zA-Z0-9_]*
// pub(crate ) fn is_valid_name(s : &str) -> bool{
//     lazy_static! {
//         static ref RE : Regex = Regex::new(r"^[@a-z_][a-zA-Z0-9_]*$").unwrap();
//     }
//     RE.is_match(s)
// }


///[@a-z_][a-zA-Z0-9_]*
pub(crate) fn analyze_name(s : &str) -> Option<Captures>{
    static RE : Lazy<Regex> = Lazy::new(|| Regex::new(r"^([@a-z_][a-zA-Z0-9_]*)([!?]*)$").unwrap());
    RE.captures(s)
}

pub(crate) fn value_type_and_name(s : &str) -> Option<(VarType, String)>{
    if let Some(cap) = analyze_name(s){
        let name = cap[1].to_string();
        let suffix = &cap[2];
        let value_type = if suffix == "!?" || suffix == "?!"{
            VarType::UndefNullable
        } else if suffix == "!"{
            VarType::Undefiable
        } else if suffix == "?"{
            VarType::Nullable
        } else if suffix == ""{
            VarType::Normal
        } else{
            return None;
        };
        return Some((value_type, name))
    } else{
        return None;
    }
}
//
/////[a-z_][a-zA-Z0-9]*\??
//pub(crate ) fn is_possible_name(s : &str) -> Option<(bool, &str)>{
//    let (b,s) = is_nullable(s);
//    if is_valid_name(s){
//        return Some((b,s))
//    } else{
//        return None;
//    }
//}