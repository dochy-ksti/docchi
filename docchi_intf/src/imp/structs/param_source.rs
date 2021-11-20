use docchi_core::structs::{VarType, ParamType};
use crate::imp::structs::source_builder::SourceBuilder;
use crate::imp::util::to_type_name::to_snake_name;
use crate::imp::util::with_old::with_old;
use crate::imp::util::with_var::with_var;
use crate::imp::util::into_qv::into_qv;

#[repr(C)] #[derive(Debug, PartialEq)]
pub(crate) struct ParamSource{
    name : String,
    var_type : VarType,
    param_type : ParamType,
    is_old : bool,
}
impl ParamSource{
    pub(crate) fn new(name : String, var_type : VarType, param_type : ParamType, is_old : bool) -> ParamSource{
        ParamSource{
            name, var_type, param_type, is_old
        }
    }

    pub fn name(&self) -> &str{
        &self.name
    }
    pub fn var_type(&self) -> VarType{
        self.var_type
    }
    pub fn param_type(&self) -> ParamType{
        self.param_type
    }
    pub fn is_old(&self) -> bool{ self.is_old }

    pub fn get(&self, mod_name : &str) -> String{
        let mut sb = SourceBuilder::new();

        let id = self.name();
        let snake_name = to_snake_name(id);
        let is_old = self.is_old();
        let var_type = self.var_type();
        let pt = self.param_type();

        sb.push(0,&format!("pub fn {}(&self) -> {}{{", with_old(&snake_name, is_old), with_var(pt.typename(), var_type)));
        sb.push(1,&format!("let qv = {}::get_{}(self.ptr, \"{}\").unwrap();", mod_name, pt.nickname(), id));


        match &var_type {
            VarType::Normal => {
                sb.push(1,&format!("qv.into_value().unwrap()"));
            },
            VarType::Undefiable => {
                sb.push(1,&format!("UndefOr::from_qv(qv).unwrap()"));
            },
            VarType::Nullable => {
                sb.push(1,&format!("NullOr::from_qv(qv).unwrap()"));
            },
            VarType::UndefNullable => {
                sb.push(1,&format!("qv"));
            },
        }
        sb.push(0,"}");
        sb.to_string()
    }
    pub fn get_immutable(&self, mod_name : &str) -> String{
        let mut sb = SourceBuilder::new();

        let id = self.name();
        let snake_name = to_snake_name(id);
        let is_old = self.is_old();
        let var_type = self.var_type();
        let pt = self.param_type();

        sb.push(0,&format!("pub fn {}(&self) -> {}{{", with_old(&snake_name, is_old), with_var(&format!("&{}", pt.typename()), var_type)));
        sb.push(1,&format!("let qv = {}::get_immutable_{}(self.ptr, \"{}\").unwrap();", mod_name, pt.nickname(), id));


        match &var_type {
            VarType::Normal => {
                sb.push(1,&format!("qv.into_value().unwrap()"));
            },
            VarType::Undefiable => {
                sb.push(1,&format!("UndefOr::from_qv(qv).unwrap()"));
            },
            VarType::Nullable => {
                sb.push(1,&format!("NullOr::from_qv(qv).unwrap()"));
            },
            VarType::UndefNullable => {
                sb.push(1,&format!("qv"));
            },
        }
        sb.push(0,"}");
        sb.to_string()
    }
    pub fn get_mutable(&self, mod_name : &str) -> String{
        let mut sb = SourceBuilder::new();

        let id = self.name();
        let snake_name = to_snake_name(id);
        let is_old = self.is_old();
        let var_type = self.var_type();
        let pt = self.param_type();

        sb.push(0,&format!("pub fn {}_mut(&mut self) -> {}{{", with_old(&snake_name, is_old), with_var(&format!("&mut {}", pt.typename()), var_type)));
        sb.push(1,&format!("let qv = {}::get_mutable_{}(self.ptr, \"{}\").unwrap();", mod_name, pt.nickname(), id));


        match &var_type {
            VarType::Normal => {
                sb.push(1,&format!("qv.into_value().unwrap()"));
            },
            VarType::Undefiable => {
                sb.push(1,&format!("UndefOr::from_qv(qv).unwrap()"));
            },
            VarType::Nullable => {
                sb.push(1,&format!("NullOr::from_qv(qv).unwrap()"));
            },
            VarType::UndefNullable => {
                sb.push(1,&format!("qv"));
            },
        }
        sb.push(0,"}");
        sb.to_string()
    }
    pub fn get_def(&self, mod_name : &str) -> String{
        let mut sb = SourceBuilder::new();

        let id = self.name();
        let snake_name = to_snake_name(id);
        let is_old = self.is_old();
        let var_type = self.var_type();
        let pt = self.param_type();

        sb.push(0,&format!("pub fn {}_def_val(&self) -> {}{{", with_old(&snake_name, is_old), with_var(pt.typename(), var_type)));
        sb.push(1,&format!("let qv = {}::get_{}_def(self.ptr, \"{}\").unwrap();", mod_name, pt.nickname(), id));

        match &var_type {
            VarType::Normal => {
                sb.push(1,&format!("qv.into_value().unwrap()"));
            },
            VarType::Undefiable => {
                sb.push(1,&format!("UndefOr::from_qv(qv).unwrap()"));
            },
            VarType::Nullable => {
                sb.push(1,&format!("NullOr::from_qv(qv).unwrap()"));
            },
            VarType::UndefNullable => {
                sb.push(1,&format!("qv"));
            },
        }
        sb.push(0,"}");
        sb.to_string()
    }

    pub fn get_def_immutable(&self, mod_name : &str) -> String{
        let mut sb = SourceBuilder::new();

        let id = self.name();
        let snake_name = to_snake_name(id);
        let is_old = self.is_old();
        let var_type = self.var_type();
        let pt = self.param_type();

        sb.push(0,&format!("pub fn {}_def_val(&self) -> {}{{", with_old(&snake_name, is_old), with_var(&format!("&{}", pt.typename()), var_type)));
        sb.push(1,&format!("let qv = {}::get_{}_def(self.ptr, \"{}\").unwrap();", mod_name, pt.nickname(), id));

        match &var_type {
            VarType::Normal => {
                sb.push(1,&format!("qv.into_value().unwrap()"));
            },
            VarType::Undefiable => {
                sb.push(1,&format!("UndefOr::from_qv(qv).unwrap()"));
            },
            VarType::Nullable => {
                sb.push(1,&format!("NullOr::from_qv(qv).unwrap()"));
            },
            VarType::UndefNullable => {
                sb.push(1,&format!("qv"));
            },
        }
        sb.push(0,"}");
        sb.to_string()
    }

    pub fn set(&self, mod_name : &str) -> String {
        let mut sb = SourceBuilder::new();

        let id = self.name();
        let snake_name = to_snake_name(id);
        let is_old = self.is_old();
        let var_type = self.var_type();
        let pt = self.param_type();

        sb.push(0, &format!("pub fn set_{}(&mut self, {} : {}){{", with_old(&snake_name, is_old), &snake_name, with_var(pt.typename(), var_type)));
        sb.push(1, &format!("{}::set_{}(self.ptr, \"{}\", {});", mod_name, pt.nickname(), id, &into_qv(&snake_name, var_type)));
        sb.push(0, "}");
        sb.to_string()
    }
    // pub(crate ) fn c_get(&self) -> Option<&str>{
    //     unimplemented!()
    // }
    // pub(crate ) fn c_set(&self) -> Option<&str>{
    //     unimplemented!()
    // }
}
