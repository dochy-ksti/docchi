use docchi_core::structs::{VarType};
use crate::imp::structs::source_builder::SourceBuilder;
use crate::imp::util::to_type_name::{to_snake_name, to_citem_type_name, to_ids_type_name};
use crate::imp::util::with_old::with_old;
use crate::imp::util::with_var::with_var;
use docchi_core::intf::ref_desc::RefDesc;

#[repr(C)] #[derive(Debug, PartialEq)]
pub(crate) struct RefSource{
    name : String,
    var_type : VarType,
    is_old : bool,
}
impl RefSource{
    pub fn new(name : String, var_type : VarType, is_old : bool) -> RefSource{
        RefSource{
            name, var_type, is_old
        }
    }
    pub fn from(desc : &RefDesc) -> RefSource{
        RefSource::new(
        desc.data_name().to_string(),
        desc.var_type(),
        desc.is_old())
    }

    pub fn name(&self) -> &str{
        &self.name
    }
    pub fn var_type(&self) -> VarType{
        self.var_type
    }
    pub fn is_old(&self) -> bool{ self.is_old }

    pub fn get(&self, from_citem : bool) -> String{
        let mut sb = SourceBuilder::new();

        let id = self.name();
        let snake_name = to_snake_name(id);
        let is_old = self.is_old();
        let var_type = self.var_type();
        let item_type_name = to_citem_type_name(id);
        let mod_name = if from_citem{ "citem" } else{ "mitem" };
        sb.push(0, &format!("pub fn ref_{}(&self) -> {}{{", with_old(&snake_name, is_old), with_var(&item_type_name, var_type)));
        sb.push(1,&format!("let qv = {}::get_ref(self.ptr, \"{}\").unwrap();", mod_name, id));
        let s = match var_type{
            VarType::Normal => format!("{}::from(qv.into_value().unwrap())", item_type_name),
            VarType::Nullable => format!("NullOr::from_qv(qv).unwrap().map(|p| {}::from(p))", item_type_name),
            VarType::Undefiable => format!("UndefOr::from_qv(qv).unwrap().map(|p| {}::from(p))", item_type_name),
            VarType::UndefNullable => format!("qv.map(|p| {}::from(p))", item_type_name),
        };
        sb.push(1, &s);
        sb.push(0, "}");
        sb.push(0, &format!("pub fn ref_id_{}(&self) -> {}{{", with_old(&snake_name, is_old), with_var("&String", var_type)));
        sb.push(1,&format!("let qv = {}::get_ref_id(self.ptr, \"{}\").unwrap();", mod_name, id));
        let s = match var_type{
            VarType::Normal => format!("qv.into_value().unwrap()"),
            VarType::Nullable => format!("NullOr::from_qv(qv).unwrap()"),
            VarType::Undefiable => format!("UndefOr::from_qv(qv).unwrap()"),
            VarType::UndefNullable => format!("qv"),
        };
        sb.push(1, &s);
        sb.push(0, "}");
        sb.to_string()
    }
    pub fn set(&self) -> String {
        let mut sb = SourceBuilder::new();
        let id = self.name();
        let snake_name = to_snake_name(id);
        let ids_type_name = to_ids_type_name(id);
        let is_old = self.is_old();
        let var_type = self.var_type;
        sb.push(0, &format!("pub fn set_ref_{}(&mut self, id : {}){{", with_old(&snake_name, is_old), with_var(&ids_type_name, var_type)));

        let exp = if var_type == VarType::Normal {
            format!("Qv::Val(id.to_str().to_string())")
        } else {
            format!("id.into_qv().map(|v| v.to_str().to_string())")
        };
        sb.push(1,&format!("mitem::set_ref(self.ptr, \"{}\", {});", id, exp));
        sb.push(0,"}");
        sb.to_string()
    }
    // pub(crate ) fn c_get(&self) -> Option<&str>{
    //     unimplemented!()
    // }
    // pub(crate ) fn c_set(&self) -> Option<&str>{
    //     unimplemented!()
    // }
}

// pub(crate ) fn var_from_qv(vt : VarType, exp : &str, item_type : &str) -> String{
//     NullOr::from_qv(qv).unwrap().map(|p| RefedCItem::from(*p))
//
// }
