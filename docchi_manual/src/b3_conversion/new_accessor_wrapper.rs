use docchi::core::structs::{UndefOr, NullOr};
use crate::b3_conversion::new_accessor::RootIntf;

pub(crate) struct NewWrapper {
    root : RootIntf
}

impl NewWrapper {
    pub fn new(root: RootIntf) -> NewWrapper { NewWrapper { root } }

    pub fn new_value(&self) -> i64 {
        Self::new_value_impl(&self.root)
    }

    fn new_value_impl(root: &RootIntf) -> i64 {
        match root.new_value() {
            //If data is the old version, the variable "new_value" will be "undefined" because it was not defined at the time.
            UndefOr::Undefined => {
                match root.old_value(){
                    NullOr::Null => root.new_value_def_val().into_value().unwrap(),
                    NullOr::Val(v) => v * 10, //new_value is ten times bigger than the old value
                }
            },
            UndefOr::Val(v) => {
                v
            }
        }
    }
}

