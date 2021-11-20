use crate::imp::structs::util::identity_equal_trait::IdentityEqual;

#[derive(Debug, PartialEq)]
pub enum Qv<T>{ Val(T), Undefined, Null }

impl<T> Qv<T>{
    pub fn qv_type(&self) -> QvType{
        match self{
            Qv::Val(_) => QvType::Val,
            Qv::Null => QvType::Null,
            Qv::Undefined => QvType::Undefined,
        }
    }

    pub fn value(&self) -> Option<&T>{
        match self{
            Qv::Val(v) => Some(v),
            Qv::Null => None,
            Qv::Undefined => None,
        }
    }

    pub fn as_ref(&self) -> Qv<&T>{
        match self{
            Qv::Val(v) => Qv::Val(v),
            Qv::Null => Qv::Null,
            Qv::Undefined => Qv::Undefined,
        }
    }

    // pub fn map<U>(&self, f : impl Fn(&T) -> U) -> Qv<U> {
    //     match self {
    //         Qv::Val(v) => Qv::Val(f(v)),
    //         Qv::Null => Qv::Null,
    //         Qv::Undefined => Qv::Undefined
    //     }
    // }

    pub fn map<U>(self, f : impl Fn(T) -> U) -> Qv<U>{
        match self {
            Qv::Val(v) => Qv::Val(f(v)),
            Qv::Null => Qv::Null,
            Qv::Undefined => Qv::Undefined
        }
    }

    pub fn opt_map<U>(&self, f : impl Fn(&T) -> Option<U>) -> Option<Qv<U>>{
        match self {
            Qv::Val(v) => f(v).map(|r| Qv::Val(r)),
            Qv::Null => Some(Qv::Null),
            Qv::Undefined => Some(Qv::Undefined)
        }
    }

    pub fn into_value(self) -> Option<T>{
        match self{
            Qv::Val(v) => Some(v),
            _ => None,
        }
    }

    pub fn into_qv(self) -> Qv<T>{ self }
}

impl<T : Clone> Clone for Qv<T>{
    fn clone(&self) -> Self {
        match self{
            Qv::Null => Qv::Null,
            Qv::Undefined => Qv::Undefined,
            Qv::Val(v) => Qv::Val(v.clone())
        }
    }
}

impl<T : IdentityEqual> IdentityEqual for Qv<T>{
    fn identity_eq(&self, other: &Self) -> bool {
        match self{
            Qv::Val(v) =>{
                if let Qv::Val(o) = other{
                    v.identity_eq(o)
                } else{ false }
            },
            Qv::Null => if let Qv::Null = other{ true } else { false }
            Qv::Undefined => if let Qv::Undefined = other{ true } else { false }
        }
    }
}


pub enum QvType{
    Val, Undefined, Null
}

impl Qv<String>{
    ///null undefined "value" のどれか
    pub(crate) fn js_string(&self) -> String{
        match self{
            Qv::Val(s) => format!(r#""{}""#,s.to_string()),
            Qv::Null => "null".to_string(),
            Qv::Undefined => "undefined".to_string(),
        }
    }
}