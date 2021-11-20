use crate::imp::structs::list_def_obj::ListDefObj;

#[derive(Debug,  Clone, PartialEq)]
pub struct MutListDef {
    default: Box<ListDefObj>,
    undefiable: bool,
    //compatible : Box<HashS<String>>,
}

impl MutListDef {
    pub(crate) fn new(default: ListDefObj, undefiable: bool) -> MutListDef {
        MutListDef { default: Box::new(default), undefiable }
    }
    pub fn default(&self) -> &ListDefObj{ self.default.as_ref() }
    pub fn undefiable(&self) -> bool{ self.undefiable }
    //pub(crate) fn compatible(&self) -> &HashS<String>{ self.compatible.as_ref() }
}

