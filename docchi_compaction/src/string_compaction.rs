use crate::kval_enum::{KVal, Decimal};
use regex::{Regex, Match};
use crate::enc_dec::decode::decode;
//use lazy_static::lazy_static;
use anyhow::Result;
use crate::enc_dec::encode_to_vec::encode_to_vec;
use once_cell::sync::Lazy;

/// Get KVals from raw Strings. If the String represents a number,
/// it's converted to number binary to reduce the size.
pub fn to_kvals<I : Iterator<Item=String>>(strs : I) -> Vec<KVal>{
    strs.map(|s| to_enum(s)).collect()
}

/// Serialize the raw Strings to bytes. If the String represents a number,
/// it's converted to number binary to reduce the size.
pub fn compress_strings<I : Iterator<Item=String>>(strs : I) -> Result<Vec<u8>>{
    let vec : Vec<KVal> = to_kvals(strs);
    let encoded = encode_to_vec(&vec)?;
    return Ok(encoded);
}

pub fn decompress(mut bytes : &[u8]) -> Result<(Vec<String>, usize)>{
    let (kihons, size) = decode(&mut bytes)?;
    Ok((to_strings(&kihons), size))
}

pub fn to_strings(kihons : &Vec<KVal>) -> Vec<String>{
    kihons.iter().map(|k| to_string(k)).collect()
}

pub fn to_string(k : &KVal) -> String{
    fn s(t : &str) -> String{ t.to_string() }

    return match k{
        KVal::Null => s("null"),
        KVal::Bit(b) => if *b{ s("1") } else{ s("0") }
        KVal::Bool(b) => if *b{ s("true")} else{ s("false") }
        KVal::Byte(b) => b.to_string(),
        KVal::Str16(s) => s.to_string(),
        KVal::Int(i) => i.to_string(),
        KVal::Float(f) => f.to_string(),
        KVal::Str256(s) => s.to_string(),
        KVal::Double(d) => d.to_string(),
        KVal::Decimal(d) => d.to_string(),
        KVal::BigStr(s) => s.to_string(),
        KVal::Binary(v) => format!("{:?}", v),
        KVal::BinaryArc(v) => format!("{:?}", v),
        KVal::Binary8(v) => format!("{:?}", v),
        KVal::Binary4(v) => format!("{:?}", v),
        KVal::Binary2(v) => format!("{:?}", v),
        KVal::Undefined(u) => format!("undefined{}", u),
    }
}

fn to_enum(string : String) -> KVal {
    let s = &string;
    if s == "0"{ return KVal::Bit(false) }
    if s == "1"{ return KVal::Bit(true) }
    if s == "true"{ return KVal::Bool(true) }
    if s == "false"{ return KVal::Bool(false) }

    if let Some(number) = get_number(s){
        if number.number == 0 && number.minus {
            //-0はこの仕組では表現できない
            return get_str(string);
        } else if let Some(dot) = number.dot{
            return KVal::Decimal(Decimal::new(number.number, dot));
        } else{
            if number.number == number.number as i64 as i128{
                //i64で損失なく表現可能である
                let number = number.number as i64;
                if number == number as i8 as i64{
                    //i8で損失なく表現可能である
                    return KVal::Byte(number as i8);
                } else {
                    return KVal::Int(number as i64);
                }
            } else{
                return KVal::Decimal(Decimal::new(number.number, 0));
            }
        }
    } else { return get_str(string); }
}

fn get_str(s : String) -> KVal {
    if s.len() < 16{
        return KVal::Str16(s);
    } else if s.len() < 256{
        return KVal::Str256(s);
    } else{
        return KVal::BigStr(s);
    }
}

struct NumResult{
    pub minus : bool,
    pub dot : Option<u8>,
    pub number : i128,
}

fn rex(s : &str) -> Option<regex::Captures>{
    static RE: Lazy<Regex> = Lazy::new(|| regex::Regex::new(r#"^(-?)(([0-9]+)(.?)([0-9]*))$"#).unwrap());

    return RE.captures(s);
}

///dot位置が255桁まで 整数がi128までのdecimalとして表現可能かを調べる。leading zerosは認めないか、0.のときの一個のみ。
/// 最後が.である数値も認めない。
fn get_number(s : &str) -> Option<NumResult>{

    if let Some(caps) = rex(s){
        let minus = (&caps[1]).len() != 0;
        fn get_dot_info(m : Option<Match>, len : usize) -> Result<Option<u8>, ()>{
            if let Some(m) = m{
                if m.as_str().len() != 0 {
                    let pos = len - m.start() - 1;
                    if pos < 256 {
                        return Ok(Some(pos as u8));
                    } else{
                        return Err(());
                    }
                }
            }
            return Ok(None);
        }

        let dot = get_dot_info(caps.get(4), s.len()).ok()?;

        let num_str = format!("{}{}{}",&caps[1], &caps[3], &caps[5]);

        let number = num_str.parse::<i128>().ok()?;
        let result = NumResult{ minus, dot, number };

        let s = &caps[2];
        if s.ends_with("."){
            return None;
        }
        if s.starts_with("0"){
            if s.starts_with("0."){
                return Some(result);
            } else{
                return None;
            }
        } else{
            return Some(result);
        }
    } else{ return None; }
}