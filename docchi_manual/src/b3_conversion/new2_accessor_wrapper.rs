
use docchi::core::structs::{UndefOr, NullOr, Qv};
use crate::b3_conversion::new2_accessor::RootIntf;

pub(crate) struct New2Wrapper {
    root: RootIntf,
}

impl New2Wrapper {
    pub fn new(root: RootIntf) -> New2Wrapper { New2Wrapper { root } }

    pub fn new_value2(&self) -> i64 {
        Self::new_value2_impl(&self.root)
    }

    fn new_value_impl(root: &RootIntf) -> NullOr<i64> {
        match root.new_value() {
            // We call a value "Qv" which can be "null" or "undefined". Maybe Qv stands for "Questionable value"
            Qv::Undefined => {
                root.old_value().map(|old| old * 10)
            },
            Qv::Null => NullOr::Null,
            Qv::Val(v) => { NullOr::Val(v) }
        }
    }

    fn new_value2_impl(root : &RootIntf) -> i64{
        match root.new_value2(){
            UndefOr::Undefined =>{
                match Self::new_value_impl(root){
                    //This value hasn't been updated yet. Returns the current default value.
                    NullOr::Null => root.new_value2_def_val().into_value().unwrap(),
                    //The value has been updated in older version's data. Convert the value to the current version
                    NullOr::Val(v) =>{
                        v * 3
                    }
                }
            },
            UndefOr::Val(v) =>{
                v
            },
        }
    }
}