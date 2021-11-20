use crate::imp::structs::qv::Qv;
#[derive(Debug, PartialEq)]
pub enum NullOr<T>{
    Val(T),
    Null
}
#[derive(Debug, PartialEq)]
pub enum UndefOr<T>{
    Val(T),
    Undefined,
}

impl<T : Clone> Clone for NullOr<T>{
    fn clone(&self) -> Self { self.as_ref().map(|t| t.clone()) }
}

impl<T : Clone> Clone for UndefOr<T>{
    fn clone(&self) -> Self {
        self.as_ref().map(|t| t.clone())
    }
}

impl<T : Copy> Copy for NullOr<T>{}
impl<T : Copy> Copy for UndefOr<T>{}

impl<T> From<Option<T>> for NullOr<T>{
    fn from(opt: Option<T>) -> Self { NullOr::from_opt(opt) }
}
impl<T> From<Option<T>> for UndefOr<T> {
    fn from(opt: Option<T>) -> Self { UndefOr::from_opt(opt) }
}
impl<T> From<NullOr<T>> for Option<T>{
    fn from(n: NullOr<T>) -> Self { n.into_value() }
}
impl<T> From<UndefOr<T>> for Option<T> {
    fn from(u: UndefOr<T>) -> Self { u.into_value() }
}

impl<T> NullOr<T>{
    pub fn as_ref(&self) -> NullOr<&T>{
        match self{
            NullOr::Val(v) => NullOr::Val(v),
            NullOr::Null => NullOr::Null,
        }
    }

    pub fn map<U>(self, f : impl FnOnce(T) -> U) -> NullOr<U> {
        match self {
            NullOr::Val(v) => NullOr::Val(f(v)),
            NullOr::Null => NullOr::Null,
        }
    }

    pub fn opt_map<U>(self, f : impl FnOnce(T) -> Option<U>) -> Option<NullOr<U>>{
        match self {
            NullOr::Val(v) => f(v).map(|r| NullOr::Val(r)),
            NullOr::Null => Some(NullOr::Null),
        }
    }

    pub fn from_qv(v : Qv<T>) -> Option<NullOr<T>>{
        match v{
            Qv::Val(v) => Some(NullOr::Val(v)),
            Qv::Null => Some(NullOr::Null),
            Qv::Undefined => None,
        }
    }

    pub fn into_qv(self) -> Qv<T>{
        match self{
            NullOr::Val(v) => Qv::Val(v),
            NullOr::Null => Qv::Null,
        }
    }

    pub fn into_value(self) -> Option<T>{
        match self{
            NullOr::Val(v) => Some(v),
            _ => None,
        }
    }

    pub fn from_opt(opt : Option<T>) -> NullOr<T>{
        match opt{
            Some(v) => NullOr::Val(v),
            None => NullOr::Null,
        }
    }
}



impl<T> UndefOr<T>{
    pub fn as_ref(&self) -> UndefOr<&T>{
        match self{
            UndefOr::Val(v) => UndefOr::Val(v),
            UndefOr::Undefined => UndefOr::Undefined,
        }
    }

    pub fn map<U>(self, f : impl FnOnce(T) -> U) -> UndefOr<U> {
        match self {
            UndefOr::Val(v) => UndefOr::Val(f(v)),
            UndefOr::Undefined => UndefOr::Undefined,
        }
    }


    pub fn opt_map<U>(self, f : impl FnOnce(T) -> Option<U>) -> Option<UndefOr<U>>{
        match self {
            UndefOr::Val(v) => f(v).map(|r| UndefOr::Val(r)),
            UndefOr::Undefined => Some(UndefOr::Undefined),
        }
    }

    pub fn from_qv(v : Qv<T>) -> Option<UndefOr<T>>{
        match v{
            Qv::Val(v) => Some(UndefOr::Val(v)),
            Qv::Undefined => Some(UndefOr::Undefined),
            Qv::Null => None,
        }
    }

    pub fn into_qv(self) -> Qv<T>{
        match self{
            UndefOr::Val(v) => Qv::Val(v),
            UndefOr::Undefined => Qv::Undefined,
        }
    }

    pub fn into_value(self) -> Option<T>{
        match self{
            UndefOr::Val(v) => Some(v),
            _ => None,
        }
    }

    pub fn from_opt(opt : Option<T>) -> UndefOr<T>{
        match opt{
            Some(v) => UndefOr::Val(v),
            None => UndefOr::Undefined,
        }
    }
}