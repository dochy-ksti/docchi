use crate::sample_test::sample_code::ref2_accessor::ListMItem;
use docchi::core::structs::NullOr;

pub(crate) struct Ref2Wrapper{
    item : ListMItem
}

impl Ref2Wrapper {
    pub fn new(item: ListMItem) -> Ref2Wrapper { Ref2Wrapper { item } }

    pub fn foo(&self) -> i64{
        match self.item.foo(){
            NullOr::Null =>{
                // When it's null, the referenced value is returned
                self.item.ref_table_a().foo()
            },
            NullOr::Val(v) =>{
                // If it's updated, the updated value is returned
                v
            }
        }
    }
}
