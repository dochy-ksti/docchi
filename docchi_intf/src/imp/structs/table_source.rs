use crate::imp::structs::source_builder::SourceBuilder;
use crate::imp::util::to_type_name::{to_snake_name, to_table_type_name, to_ids_type_name, to_citem_type_name, to_type_name};
use crate::imp::util::with_old::with_old;
use crate::imp::structs::citem_source::CItemSource;
use docchi_core::intf::member_desc::{MemberDesc, KeyItem};

#[derive(Debug, PartialEq)]
pub(crate) struct TableSource {
    stem : String,
    is_old : bool,
    keys : Vec<KeySource>,
    item_source : CItemSource,
}
#[derive(Debug, PartialEq)]
pub(crate) struct KeySource{
    key : String,
    is_old : bool,
}
impl KeySource{
    pub fn new(key : String, is_old : bool) -> KeySource{ KeySource{ key, is_old }}
    pub fn from(key : &KeyItem) -> KeySource{
        KeySource::new(key.key().to_string(), key.is_old())
    }
    pub fn key_name(&self) -> String{
        if self.is_old{ format!("{}_old", to_snake_name(&self.key)) } else{ to_snake_name(&self.key) }
    }
    pub fn enum_name(&self) -> String{
        if self.is_old{ format!("{}Old", to_type_name(&self.key)) } else{ to_type_name(&self.key) }
    }
}
impl TableSource {
    pub fn new(stem : String, is_old : bool, keys : Vec<KeySource>, item_source : CItemSource) -> TableSource {
        TableSource { stem, is_old, keys, item_source }
    }
    pub fn from(desc : &MemberDesc) -> TableSource {
        let mut keys: Vec<KeySource> = vec![];
        let cs = desc.child_descs().unwrap();
        for key in cs.keys() {
            keys.push(KeySource::from(key))
        }
        TableSource::new(
            desc.name().to_string(),
            desc.is_old(),
            keys,
            CItemSource::from(desc.name().to_string(), cs.items(), cs.refs())
        )
    }

    pub fn stem(&self) -> &str{ &self.stem }
    pub fn is_old(&self) -> bool{ self.is_old }
    //pub fn keys(&self) -> &[KeySource]{ &self.keys }
    //pub fn item_source(&self) -> &CItemSource { &self.item_source }

    pub fn get(&self) -> String{
        let mut sb = SourceBuilder::new();

        let id = self.stem();
        let snake_name = to_snake_name(id);
        let is_old = self.is_old();
        let data_type_name = to_table_type_name(id);
        let fn_name = with_old(&snake_name, is_old);
        // sb.push(0,&format!("pub unsafe fn {}_us(&self) -> {}{{", &fn_name , &data_type_name));
        // sb.push(1,&format!("let ans = root::get_table(self.ptr, \"{}\").unwrap();", id));
        // sb.push(1,&format!("{}::new(ans)", &data_type_name));
        // sb.push(0,"}");
        sb.push(0,&format!("pub fn {}(&self) -> CTableConst<{}>{{", &fn_name, &data_type_name));
        sb.push(1,&format!("let t = {}::new(root::get_table(self.ptr.def(), \"{}\").unwrap());", &data_type_name, id));
        sb.push(1,&format!("CTableConst::new(t, self)"));
        sb.push(0,"}");
        sb.to_string()
    }
    pub fn to_string(&self) -> String{
        let mut sb = SourceBuilder::new();
        let id = self.stem();
        let data_type_name = to_table_type_name(id);
        let item_type_name = to_citem_type_name(id);
        let ids_type_name = to_ids_type_name(id);

        sb.push(0,&format!("#[derive(Debug, PartialEq)]"));
        sb.push(0,&format!("pub struct {} {{", &data_type_name));
        sb.push(1,"ptr : TablePtr,");
        sb.push(0,"}");
        sb.push(0, &format!("impl {} {{", &data_type_name));
        sb.push(1, &format!("pub fn new(ptr : TablePtr) -> {}{{ {}{{ ptr }} }} ",
                           &data_type_name, &data_type_name));

        for key in &self.keys {
            let key_name = key.key_name();
            sb.push(1,&format!("pub unsafe fn {}_us(&self) -> {} {{", &key_name, &item_type_name));
            sb.push(2,&format!("let ptr = table::get_value(self.ptr, \"{}\").unwrap();", &key.key));
            sb.push(2, &format!("{}::from(ptr)", &item_type_name));
            sb.push(1, "}");

            sb.push(1,&format!("pub fn {}(&self) -> CItemConst<{}> {{", &key_name, &item_type_name));
            sb.push(2,&format!("let ptr = table::get_value(self.ptr, \"{}\").unwrap();", &key.key));
            sb.push(2,&format!("CItemConst::new({}::from(ptr), self)", &item_type_name));
            sb.push(1, "}");
        }

        if self.keys.len() != 0 {
            sb.push(1,&format!("pub unsafe fn get_by_id_us(&self, id : {}) -> {}{{", &ids_type_name, &item_type_name));
            sb.push(2,"match id{");

            for key in &self.keys {
                 sb.push(3,&format!("{}::{} => self.{}_us(),", &ids_type_name, &key.enum_name(), &key.key_name()));
            }
            sb.push(2, "}");
            sb.push(1, "}");

            sb.push(1,&format!("pub fn get_by_id(&self, id : {}) -> CItemConst<{}>{{", &ids_type_name, &item_type_name));
            sb.push(2,"CItemConst::new(unsafe{ self.get_by_id_us(id) }, self)");
            sb.push(1, "}");
        }

        sb.push(0, "}");


        sb.append(&format!("#[repr(u64)] pub enum {}{{ ", &ids_type_name));
        for key in &self.keys{
            sb.append(&format!("{}, ", &key.enum_name()))
        }
        sb.push(0, "}");
        sb.push(0, &format!("impl {}{{", &ids_type_name));
        sb.push(1, &format!("pub fn from_str(id : &str) -> Option<Self>{{"));
        sb.push(2, &format!("match id{{"));
        for key in &self.keys {
            sb.push(3, &format!(r#""{}" => Some(Self::{}),"#, &key.key, &key.enum_name()));
        }

        sb.push(3, "_ =>{ None }");
        sb.push(2, "}");
        sb.push(1, "}");
        sb.push(1, &format!("pub fn from_num(id : usize) -> Self{{"));
        sb.push(2, &format!("match id{{"));
        for (id, key) in self.keys.iter().enumerate() {
            sb.push(3, &format!("{} => Self::{},", id, key.enum_name()));
        }
        sb.push(3, &format!("_ => panic!(\"invalid ID num {{}} {}\", id),", &ids_type_name));
        sb.push(2, "}");
        sb.push(1, "}");
        sb.push(1, &format!("pub fn len() -> usize{{ {} }}", self.keys.len()));
        sb.push(1, &format!("pub fn to_num(&self) -> usize{{"));
        sb.push(2, &format!("match self{{"));
        for (i, key) in self.keys.iter().enumerate() {
            sb.push(3, &format!(r#"{}::{} => {},"#, &ids_type_name, &key.enum_name(), i));
        }
        sb.push(2, "}");
        sb.push(1, "}");
        sb.push(1, &format!("pub fn metadata() -> &'static [&'static str]{{"));
        let mut s = format!("&[");
        for key in &self.keys {
            s.push_str("\"");
            s.push_str(&key.key);
            s.push_str("\", ");
        }
        s.push_str("]");
        sb.push(2, &s);
        sb.push(1, "}");
        sb.push(1, "pub fn to_str(&self) -> &'static str{");
        sb.push(2, "Self::metadata()[self.to_num()]");
        sb.push(1, "}");
        sb.push(0,"}");

        sb.push_without_newline(0, &self.item_source.to_string());
        sb.to_string()
    }
}