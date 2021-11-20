use crate::imp::to_member_source::{MemberSource, to_member_source};
use crate::imp::structs::source_builder::SourceBuilder;
use crate::imp::structs::ref_source::RefSource;
use docchi_core::intf::member_desc::MemberDesc;
use docchi_core::intf::ref_desc::RefDescs;
use crate::imp::util::to_type_name::to_citem_type_name;
use crate::imp::structs::refs_source::RefsSource;
use docchi_core::structs::ParamType;

#[derive(Debug, PartialEq)]
pub(crate) struct CItemSource {
    stem : String,
    members : Vec<MemberSource>,
    refs : RefsSource,
}
impl CItemSource {
    //pub fn new(members : Vec<MemberSource>, refs : RefsSource, stem : String) -> CItemSource { CItemSource { members, refs, stem } }
    pub fn from(stem : String, mems : &[MemberDesc], refs : &RefDescs) -> CItemSource {
        CItemSource {
            stem,
            members : mems.iter().map(to_member_source).collect(),
            refs : RefsSource::new(refs.items().iter().map(RefSource::from).collect(), refs.is_enum()),
        }
    }

    pub fn stem(&self) -> &str{ &self.stem }
    pub fn members(&self) -> &[MemberSource]{ &self.members }
    //pub fn refs(&self) -> &RefsSource{ &self.refs }

    pub fn to_string(&self) -> String{
        let mut sb = SourceBuilder::new();

        let id = self.stem();
        let item_type_name = to_citem_type_name(id);

        sb.push(0,&format!("#[derive(Debug, PartialEq, Clone, Copy)]"));
        sb.push(0,&format!("pub struct {} {{", &item_type_name));
        sb.push(1,"ptr : CItemPtr,");
        sb.push(0,"}");
        sb.push(0, &format!("impl From<CItemPtr> for {} {{", &item_type_name));
        sb.push(1, &format!("fn from(ptr : CItemPtr) -> Self {{ Self{{ ptr }} }}"));
        sb.push(0, &format!("}}"));
        sb.push(0, &format!("impl {} {{", &item_type_name));

        for mem in self.members() {
            match mem{
                MemberSource::Param(param) =>{
                    match param.param_type() {
                        ParamType::Binary | ParamType::String | ParamType::IntArray | ParamType::FloatArray =>{
                            //sb.push_without_newline(1, &param.get("citem"));
                            sb.push_without_newline(1, &param.get_def_immutable("citem"));
                            sb.push_without_newline(1, &param.get_immutable("citem"));
                        }
                        _ => {
                            sb.push_without_newline(1, &param.get("citem"));
                            sb.push_without_newline(1, &param.get_def("citem"));
                        }
                    }
                },
                MemberSource::Table(_) =>{},
                MemberSource::CList(_) =>{},
                MemberSource::MList(_) =>{},
                MemberSource::Cil(l) =>{
                    sb.push_without_newline(1, &l.get());
                },
                MemberSource::Mil(_) =>{},
            }
        }

        sb.push_without_newline(1, &self.refs.get(self.stem(), true));
        sb.push(0, "}");

        for mem in &self.members{
            match mem{
                MemberSource::Cil(l) =>{
                    sb.push_without_newline(0, &l.to_string());
                },
                MemberSource::CList(_) =>{},
                MemberSource::Table(_) =>{},
                MemberSource::MList(_) =>{},
                MemberSource::Param(_) =>{},
                MemberSource::Mil(_) =>{},
            }
        }
        if let Some(s) = self.refs.to_string(self.stem(), true) {
            sb.push_without_newline(0, &s);
        }

        sb.to_string()
    }
}