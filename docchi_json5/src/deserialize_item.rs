use crate::error::Result;
use crate::jval::JVal;
use crate::de::{deserialize_any, Seq, Map};
use pest::Span;
use crate::de::Rule;
use std::rc::Rc;
use linked_hash_map::{LinkedHashMap, Entry};
use crate::error::MyError;
//use crate::json5_back::MyError;
//use linked_hash_map::LinkedHashMap;

pub(crate) fn get_unit(span : Span, rc : Rc<String>) -> JVal { JVal::Null(s(span, rc)) }

pub(crate) fn get_undefined(span : Span, rc : Rc<String>) -> JVal{ JVal::Undefined(s(span, rc)) }

pub(crate) fn get_bool(b: bool, span : Span, rc : Rc<String>) -> JVal { JVal::Bool(b, s(span, rc)) }

pub(crate) fn get_string(st: String, span : Span, rc : Rc<String>) -> JVal { JVal::String(st, s(span, rc)) }

pub(crate) fn get_i64(i: i64, span : Span, rc : Rc<String>) -> JVal { JVal::Int(i, s(span, rc)) }

pub(crate) fn get_f64(f: f64, span : Span, rc : Rc<String>) -> JVal { JVal::Double(f, s(span, rc)) }

pub(crate) fn get_seq(seq: Seq, span : Span, rc : Rc<String>) -> Result<JVal> {
    let mut result : Vec<JVal> = vec![];
    let pairs = seq.pairs;
    for pair in pairs{
        result.push(deserialize_any(pair, rc.clone())?);
    }
    return Ok(JVal::Array(result, s(span, rc)));
}

pub(crate) fn get_map(m: Map, span : Span, rc : Rc<String>) -> Result<JVal> {
    let mut result : LinkedHashMap<String, JVal> = LinkedHashMap::new();
    let mut pairs = m.pairs;
    loop{
        let op = pairs.next();
        if op.is_none(){ break; }
        let p = op.unwrap();
        let ident = match p.as_rule(){
            Rule::identifier =>{ p.as_str().to_string() },
            Rule::string =>{
                let s = p.as_str();
                (&s[1..s.len()-1]).to_string()
            },
            _ =>{
                //println!("{:?}",p.as_rule());
                //println!("{:?}",p.as_str());

                unreachable!()
            }
        };
        let p   = pairs.next().unwrap();
        //let span = p.as_span();
        let val = deserialize_any(p, rc.clone())?;

        match result.entry(ident){
            Entry::Occupied(o) =>{
                let start = span.start();
                let (line,_col) = span.start_pos().line_col();
                let ident = o.key();
                return Err(MyError::new(format!("{} {} is duplicated", line, ident), start));
            },
            Entry::Vacant(v) =>{
                v.insert(val);
            }
        }
        // match result.insert(ident.to_string(), val){
        //     Some(_val) =>{
        //         let start = span.start();
        //         let (line,_col) = span.start_pos().line_col();
        //         return Err(MyError::new(format!("{} {} is duplicated", line, ident), start));
        //     },
        //     _ =>{}
        // }
    }

    Ok(JVal::Map(result, s(span, rc)))
}

fn s(span : Span, rc : Rc<String>) -> crate::jval::Span{
    crate::jval::Span{ start : span.start(), end : span.end(), text : rc }
}


