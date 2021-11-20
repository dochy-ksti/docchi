use crate::imp::structs::list_type::ListType;

pub(crate) fn list_type_to_string(l : &ListType) -> String{
    let s = match l{
        ListType::Table => "Table",
        ListType::CList => "CList",
        ListType::MList => "MList",
        //ListType::InnerData => "InnerData",
        ListType::Cil => "Cil",
        ListType::Mil => "Mil",
        //ListType::InnderDataDef => "InnerDataDef",
        ListType::CilDef => "CilDef",
        //ListType::InnerMutDef => if has_item{ "__InnerViolatedListDef" } else{ "InnerMutDef" },
    };
    s.to_string()

}