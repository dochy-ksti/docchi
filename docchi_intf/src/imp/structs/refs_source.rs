use crate::imp::structs::ref_source::RefSource;
use crate::imp::structs::source_builder::SourceBuilder;
use crate::imp::util::to_type_name::{to_enum_type_name, to_citem_type_name, to_kind_type_name, to_type_name, to_ids_type_name};

#[repr(C)] #[derive(Debug, PartialEq)]
pub(crate) struct RefsSource{
    refs : Vec<RefSource>,
    is_enum : bool,
}

impl RefsSource{
    pub fn new(refs : Vec<RefSource>, is_enum : bool) -> RefsSource{
        RefsSource{ refs, is_enum }
    }
    //pub fn is_enum(&self) -> bool{ self.is_enum }
    //pub fn refs(&self) -> &[RefSource]{ &self.refs }

    pub fn get(&self, stem : &str, from_citem : bool) -> String{
        let mut sb = SourceBuilder::new();
        let mod_name = if from_citem{ "citem" } else{ "mitem" };
        if self.is_enum {
            let enum_type_name = to_enum_type_name(stem);
            sb.push(0, &format!("pub fn get_enum(&self) -> {}{{", &enum_type_name));
            sb.push(1, &format!("let (list_name, _) = {}::get_enum(self.ptr).unwrap();", &mod_name));
            sb.push(1, &format!("let p = if let Qv::Val(p) = {}::get_ref(self.ptr, &list_name).unwrap(){{ p }} else {{ unreachable!() }};", mod_name));
            sb.push(1, &format!("{}::new(&list_name,p)", &enum_type_name));
            sb.push(0, "}");
            sb.push(0, &format!("pub fn get_enum_ids(&self) -> (String,String){{"));
            sb.push(1, &format!("{}::get_enum(self.ptr).unwrap()", &mod_name));
            sb.push(0, "}");
        } else {
            for r in &self.refs {
                sb.push_without_newline(0, &r.get(from_citem));
            }
        }
        sb.to_string()
    }

    pub fn set(&self, stem  : &str) -> String {
        let mut sb = SourceBuilder::new();

        if self.is_enum {
            sb.push(0, &format!("pub fn set_enum(&mut self, kind : {}){{", &to_kind_type_name(stem)));
            sb.push(1, &format!("let (list_name, id) = kind.id();"));
            sb.push(1, &format!("mitem::set_enum(self.ptr, list_name, id);"));
            sb.push(0, "}");
        } else{
            for r in &self.refs {
                sb.push_without_newline(0, &r.set());
            }
        }
        sb.to_string()
    }

    pub fn to_string(&self, stem : &str, from_citem : bool) -> Option<String>{
        if self.is_enum{ Some(self.get_enum_source(stem, from_citem)) } else{ None }
    }
    fn get_enum_source(&self, stem : &str, from_citem : bool) -> String{
        let mut sb = SourceBuilder::new();

        let enum_type_name = to_enum_type_name(stem);

        sb.append(&format!("pub enum {}{{ ", &enum_type_name));
        for r in &self.refs{
            let item_type = to_citem_type_name(r.name());
            sb.append(&format!("{}({}), ", &to_type_name(r.name()), &item_type));
        }
        sb.append("}\n");


        sb.push(0, &format!("impl {}{{", &enum_type_name));
        sb.push(1, &format!("pub fn new(list_name : &str, ptr : CItemPtr) -> {}{{", &enum_type_name));
        sb.push(2, &format!("match list_name{{"));
        for r in &self.refs {
            let item_type = to_citem_type_name(r.name());
            sb.push(3, &format!(r#""{}" => {}::{}({}::from(ptr)),"#, r.name(), &enum_type_name, &to_type_name(r.name()), &item_type));
        }
        sb.push(3, &format!(r#"_ => panic!("{} there's no enum type named {{}}", &list_name),"#, &enum_type_name));
        sb.push(2, "}");
        sb.push(1, "}");
        sb.push(0, "}");

        if from_citem == false {
            let kind_type_name = to_kind_type_name(stem);
            sb.append(&format!("pub enum {}{{ ", &kind_type_name));
            for r in &self.refs {
                sb.append(&format!("{}({}), ", &to_type_name(r.name()), &to_ids_type_name(r.name())));
            }
            sb.append("}\n");

            sb.push(0, &format!("impl {}{{", kind_type_name));
            sb.push(1, &format!("pub fn id(&self) -> (&'static str, &'static str){{"));
            sb.push(2, &format!("match self{{"));
            for r in &self.refs {
                sb.push(3,&format!("{}::{}(v) => (\"{}\", v.to_str()),",
                                   &kind_type_name, &to_type_name(r.name()), r.name()));
            }
            sb.push(2, "}");
            sb.push(1, "}");
            //sb.push(1, &format!("pub(crate ) fn metadata(&self) -> {{"));

            sb.push(0, "}");
        }
        sb.to_string()
    }
}

