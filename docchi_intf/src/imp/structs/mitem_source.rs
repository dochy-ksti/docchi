use crate::imp::to_member_source::{MemberSource, to_member_source};
use crate::imp::structs::source_builder::SourceBuilder;
use crate::imp::structs::ref_source::RefSource;
use docchi_core::intf::member_desc::MemberDesc;
use docchi_core::intf::ref_desc::RefDescs;
use crate::imp::util::to_type_name::to_mitem_type_name;
use crate::imp::structs::refs_source::RefsSource;
use docchi_core::structs::ParamType;

#[derive(Debug, PartialEq)]
pub(crate) struct MItemSource {
    stem : String,
    members : Vec<MemberSource>,
    refs : RefsSource,
}
impl MItemSource {
    //pub fn new(members : Vec<MemberSource>, refs : RefsSource, stem : String) -> MItemSource { MItemSource { members, refs, stem } }
    pub fn from(stem : String, mems : &[MemberDesc], refs : &RefDescs) -> MItemSource {
        MItemSource {
            stem,
            members : mems.iter().map(to_member_source).collect(),
            refs : RefsSource::new(refs.items().iter().map(RefSource::from).collect(), refs.is_enum())
        }
    }

    pub fn stem(&self) -> &str{ &self.stem }
    pub fn members(&self) -> &[MemberSource]{ &self.members }
    //pub fn refs(&self) -> &RefsSource{ &self.refs }

    pub fn to_string(&self) -> String{
        let mut sb = SourceBuilder::new();

        let id = self.stem();
        let item_type_name = to_mitem_type_name(id);

        sb.push(0,&format!("#[derive(Debug, PartialEq, Clone, Copy)]"));
        sb.push(0,&format!("pub struct {} {{", &item_type_name));
        sb.push(1,"ptr : MItemPtr,");
        sb.push(0,"}");
        sb.push(0, &format!("impl From<MItemPtr> for {} {{", &item_type_name));
        sb.push(1, &format!("fn from(ptr : MItemPtr) -> Self {{"));
        sb.push(2, &format!("Self{{ ptr }}"));
        sb.push(1, "}");
        sb.push(0,"}");
        sb.push(0, &format!("impl {} {{", &item_type_name));

        for mem in self.members() {
            match mem{
                MemberSource::Param(param) =>{

                    match param.param_type() {
                        ParamType::Binary | ParamType::String | ParamType::IntArray | ParamType::FloatArray =>{
                            //sb.push_without_newline(1, &param.get("mitem"));
                            sb.push_without_newline(1, &param.get_def_immutable("mitem"));
                            sb.push_without_newline(1, &param.get_immutable("mitem"));
                            sb.push_without_newline(1, &param.get_mutable("mitem"));
                            sb.push_without_newline(1, &param.set("mitem"));
                        }
                        _ => {
                            sb.push_without_newline(1, &param.get("mitem"));
                            sb.push_without_newline(1, &param.get_def("mitem"));
                            sb.push_without_newline(1, &param.set("mitem"));
                        }
                    }
                },
                MemberSource::Table(_) =>{},
                MemberSource::CList(_) =>{},
                MemberSource::MList(_) =>{},
                MemberSource::Cil(_l) =>{
                    //sb.push_without_newline(1, &l.get("mut_list_item", "self.ptr"));
                },
                MemberSource::Mil(m) =>{
                    sb.push_without_newline(1, &m.get());
                }
            }
        }
        sb.push_without_newline(1, &self.refs.get(self.stem(), false));
        sb.push_without_newline(1, &self.refs.set(self.stem()));

        sb.push(0, "}");

        for mem in &self.members{
            match mem{
                MemberSource::Cil(_l) =>{

                },
                MemberSource::CList(_) =>{},
                MemberSource::Table(_) =>{},
                MemberSource::MList(_) =>{},
                MemberSource::Param(_) =>{},
                MemberSource::Mil(m) =>{
                    sb.push_without_newline(0, &m.to_string());
                },
            }
        }
        if let Some(s) = self.refs.to_string(self.stem(), false) {
            sb.push_without_newline(0, &s);
        }
        sb.to_string()
    }
}