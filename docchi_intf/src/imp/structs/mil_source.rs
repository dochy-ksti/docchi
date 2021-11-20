use crate::imp::structs::source_builder::SourceBuilder;
use crate::imp::util::to_type_name::{to_snake_name, to_mitem_type_name};
use crate::imp::util::with_old::with_old;
use docchi_core::intf::member_desc::{MemberDesc};
use crate::imp::structs::mitem_source::MItemSource;
use docchi_core::structs::VarType;

#[derive(Debug, PartialEq)]
pub(crate) struct MilSource {
    stem : String,
    undefiable : bool,
    is_old : bool,
    item_source : MItemSource,
}
impl MilSource {
    pub fn new(stem : String, undefiable : bool, is_old : bool, item_source : MItemSource) -> MilSource {
        MilSource { stem, undefiable, is_old, item_source }
    }
    pub fn from(desc : &MemberDesc) -> MilSource {
        let cs = desc.child_descs().unwrap();

        let undefiable = if desc.var_type() == VarType::Undefiable{ true } else { false };

        MilSource::new(
            desc.name().to_string(),
            undefiable,
            desc.is_old(),
            MItemSource::from(desc.name().to_string(), cs.items(), cs.refs())
        )
    }

    pub fn stem(&self) -> &str{ &self.stem }
    pub fn is_old(&self) -> bool{ self.is_old }
    //pub fn item_source(&self) -> &MItemSource { &self.item_source }

    pub fn get(&self) -> String{
        let mut sb = SourceBuilder::new();

        let id = self.stem();
        let snake_name = to_snake_name(id);
        let is_old = self.is_old();
        let item_type_name = to_mitem_type_name(id);
        let fn_name = with_old(&snake_name, is_old);
        if self.undefiable{
            sb.push(0, &format!("pub fn {}(&self) -> Option<MListConst<{}>>{{", &fn_name , &item_type_name));
            sb.push(1, &format!("let mil = mitem::get_mil_const(self.ptr, \"{}\").unwrap();", id));
            sb.push(1, &format!("mil.map(move |p| MListConst::new(p, self))"));
            sb.push(0, "}");
        } else {
            sb.push(0, &format!("pub fn {}(&self) -> MListConst<{}>{{", &fn_name, &item_type_name));
            sb.push(1, &format!("let mil = mitem::get_mil_const(self.ptr, \"{}\").unwrap().unwrap();", id));
            sb.push(1, &format!("MListConst::new(mil, self)"));
            sb.push(0, "}");
        }
        sb.push(0, &format!("pub fn {}_mut(&mut self) -> MListMut<{}>{{", &fn_name , &item_type_name));
        sb.push(1, &format!("let p = mitem::get_mil_mut(self.ptr, \"{}\").unwrap();", id));
        sb.push(1, &format!("MListMut::new(p, self)"));
        sb.push(0, "}");
        sb.to_string()
    }

    pub fn to_string(&self) -> String{
        let mut sb = SourceBuilder::new();
        sb.push_without_newline(0, &self.item_source.to_string());
        sb.to_string()
    }
}