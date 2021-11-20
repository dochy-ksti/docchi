use std::sync::Arc;

/// The Value to be serialized.
#[derive(Debug, PartialEq)]
pub enum KVal {
    Null,
    Bit(bool),
    Bool(bool),
    Byte(i8),
    Str16(String),
    Int(i64),
    Float(f32),
    Str256(String),
    Double(f64),
    Decimal(Decimal),
    BigStr(String),
    Binary(Vec<u8>),
    BinaryArc(Arc<Vec<u8>>),
    Binary8(Vec<u64>),
    Binary4(Vec<u32>),
    Binary2(Vec<u16>),
    Undefined(u8),
}


/// Decimal represents 40 digits with dot.
/// It's supposed to be used for decimal strings which can be converted to without errors.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Decimal{
    /// The integer part.
    pub int : i128,
    /// The number is an integer when dot == 0. When dot == 1, the dot slides to left.
    /// If int == 128, it's 12.8. If int == 100, it's 10.0
    /// When dot == 2, if int == 128, it's 1.28. If int == 100, it's 1.00
    /// ...
    pub dot : u8,
}

impl Decimal{
    /// get the nearest f64
    pub fn to_f64(&self) -> f64 {
        crate::enc_dec::decimal_lib::to_f64(self.int, self.dot)
    }
    /// get the edecimal string
    pub fn to_string(&self) -> String{
        crate::enc_dec::decimal_lib::to_string(self.int, self.dot)
    }
    /// create the value
    pub fn new(int : i128, dot : u8) -> Decimal{
        Decimal{ int, dot }
    }
}

impl KVal {
    ///bit,byte,int,float,double can be converted to f64
    pub fn as_f64(&self) -> Option<f64>{
        use KVal::*;
        match self{
            Bit(b) => Some(*b as isize as f64),
            Byte(b) => Some(*b as f64),
            Int(i) => Some(*i as f64),
            Float(f) => Some(*f as f64),
            Double(f) => Some(*f),
            _ => None,
        }
    }

    ///bit,byte,int,float,double can be converted to f32
    pub fn as_f32(&self) -> Option<f32>{
        use KVal::*;
        match self{
            Bit(b) => Some(*b as isize as f32),
            Byte(b) => Some(*b as f32),
            Int(i) => Some(*i as f32),
            Float(f) => Some(*f),
            Double(f) => Some(*f as f32), //???
            _ => None,
        }
    }

    ///bit, byte, int can be converted to i64
    pub fn as_i64(&self) -> Option<i64>{
        use KVal::*;
        match self {
            Bit(b) => Some(*b as i64),
            Byte(b) => Some(*b as i64),
            Int(i) => Some(*i as i64),
            _ => None,
        }
    }

    ///str16, str256, BigStr, Bit <- Empty String
    pub fn as_string(&self) -> Option<String>{
        use KVal::*;
        match self{
            Bit(_b) => Some(String::new()),
            Str16(s) => Some(s.to_string()),
            Str256(s) => Some(s.to_string()),
            BigStr(s) => Some(s.to_string()),
            _ => None,
        }
    }

    /// Bool, Bit can be converted to bool
    pub fn as_bool(&self) -> Option<bool>{
        match self{
            KVal::Bool(b) => Some(*b),
            KVal::Bit(b) => Some(*b),
            _ => None,
        }
    }

    /// return Some if it's null
    pub fn as_null(&self) -> Option<()>{
        match self{
            KVal::Null => Some(()),
            _ => None,
        }
    }

    /// if it's == undefined0, return Some(0).
    /// if it's == undefined1, return Some(1).
    /// ...
    pub fn as_undefined(&self) -> Option<u8>{
        match self{
            KVal::Undefined(u) => Some(*u),
            _ => None,
        }
    }

    /// return Some(Decimal) if it's Decimal
    pub fn as_decimal(&self) -> Option<Decimal>{
        match self{
            KVal::Decimal(d) => Some(*d),
            _ => None,
        }
    }

    /// Get the string expression of this value.
    pub fn to_string(&self) -> String{
        crate::string_compaction::to_string(self)
    }
}