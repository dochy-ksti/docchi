use crate::sample_test::sample_code::version_awareness_accessor::RootIntf;
use docchi::core::structs::{UndefOr, NullOr};

pub(crate) struct VeraAccessorWrapper{
    root : RootIntf
}

impl VeraAccessorWrapper {
    pub fn new(root: RootIntf) -> VeraAccessorWrapper { VeraAccessorWrapper { root } }

    pub fn new_value(&self) -> i64 {
        Self::new_value_impl(&self.root)
    }

    fn new_value_impl(root: &RootIntf) -> i64 {
        match root.new_value() {
            //If data is old, the variable "new_value" will be "undefined" because it was not defined at the time.
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
