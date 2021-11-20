use crate::imp::structs::source_builder::SourceBuilder;
use crate::imp::util::to_type_name::{to_snake_name, to_citem_type_name};
use crate::imp::util::with_old::with_old;
use crate::imp::structs::citem_source::CItemSource;
use docchi_core::intf::member_desc::{MemberDesc};

#[derive(Debug, PartialEq)]
pub(crate) struct CListSource {
    stem : String,
    is_old : bool,
    item_source : CItemSource,
}
impl CListSource {
    pub fn new(stem : String, is_old : bool, item_source : CItemSource) -> CListSource {
        CListSource { stem, is_old, item_source }
    }
    pub fn from(desc : &MemberDesc) -> CListSource {
        let cs = desc.child_descs().unwrap();

        CListSource::new(
            desc.name().to_string(),
            desc.is_old(),
            CItemSource::from(desc.name().to_string(), cs.items(), cs.refs())
        )
    }

    pub fn stem(&self) -> &str{ &self.stem }
    pub fn is_old(&self) -> bool{ self.is_old }
    //pub fn item_source(&self) -> &CItemSource { &self.item_source }

    pub fn get(&self) -> String{
        let mut sb = SourceBuilder::new();

        let id = self.stem();
        let snake_name = to_snake_name(id);
        let is_old = self.is_old();
        let item_type_name = to_citem_type_name(id);
        let fn_name = with_old(&snake_name, is_old);
        //sb.push(0,&format!("pub unsafe fn {}_us(&self) -> CListPtr<{}>{{", &fn_name, &item_type_name));
        //sb.push(1,&format!("root::get_clist(self.ptr, \"{}\").unwrap()", id));
        //sb.push(0,"}");
        sb.push(0,&format!("pub fn {}(&self) -> CListConst<{}>{{", &fn_name, &item_type_name));
        sb.push(1,&format!("CListConst::new(root::get_clist(self.ptr, \"{}\").unwrap(), self)", id));
        sb.push(0,"}");
        sb.to_string()
    }
    pub fn to_string(&self) -> String{
        let mut sb = SourceBuilder::new();
        sb.push_without_newline(0, &self.item_source.to_string());
        sb.to_string()
    }
}