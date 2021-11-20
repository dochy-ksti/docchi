use crate::imp::structs::var_type::VarType;
use crate::imp::structs::qv::Qv;
use crate::IdentityEqual;

#[derive(Debug, PartialEq, Clone)]
pub struct RefValue{
    v : Box<RefValueInternal>,
}

#[derive(Debug, PartialEq, Clone)]
struct RefValueInternal{
    value : Qv<String>,
    value_type : VarType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RefSabValue{
    value : Box<Qv<String>>,
}

impl RefValue{
    pub(crate) fn new(value : Qv<String>, value_type : VarType) -> RefValue{
        RefValue{ v : Box::new(RefValueInternal{value, value_type }) }
    }
    pub(crate) fn var_type(&self) -> VarType { self.v.value_type }
    pub fn value(&self) -> &Qv<String>{ &self.v.value }
    pub(crate) fn into_sab_value(self) ->RefSabValue{ RefSabValue::new(self.value().clone()) }

    ///右の値を自身に代入できるか
    pub(crate) fn acceptable(&self, other : &RefSabValue) -> bool{
        self.var_type().acceptable(&other.value.qv_type())
    }

    // ///右が取りうる値すべてが左に代入できるか
    // pub(crate) fn compatible(&self, other : &Self) -> bool{
    //     self.value_type().compatible(&other.value_type())
    // }


}
impl RefSabValue{
    pub fn new(value : Qv<String>) -> RefSabValue{ RefSabValue{ value : Box::new(value) } }
    pub fn value(&self) -> &Qv<String>{ &self.value }
    pub(crate) fn into_ref_value_for_json(self) -> RefValue{
        //sabun側は?とか!とかなくていいのでNormalでよい
        RefValue::new(self.value().clone(), VarType::Normal)
    }
}

impl IdentityEqual for RefSabValue{
    fn identity_eq(&self, other: &Self) -> bool {
        self.value.identity_eq(other.value())
    }
}