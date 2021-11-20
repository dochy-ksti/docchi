use crate::error::CoreResult;
use std::str::from_utf8;
use std::path::Path;
use crate::imp::json_to_rust::json_name::{json_name, NameType};
use crate::imp::json_to_rust::{json_item_str_to_rust, json_root_to_rust};
use crate::imp::structs::dochy_archive::ArchivingItem;


pub(crate) fn json_file_to_rust(path : &str, dat : &[u8]) -> CoreResult<ArchivingItem>{
    let s = from_utf8(dat)?;
    let path = Path::new(path);
    let file_name = path.file_stem().ok_or_else(|| format!("Invalid Path {:?}", path))?
        .to_str().ok_or_else(|| format!("Invalid Filename {:?}", path))?;
    if file_name == "root"{
        return Ok(ArchivingItem::Root(json_root_to_rust(s)?));
    }
    let name_type = json_name(file_name).ok_or_else(|| format!("Invalid Filename {:?}", file_name))?;
    match name_type {
        NameType::Name(name, var_type) => {
            match json_item_str_to_rust(s, &name, var_type.clone()) {
                Ok(val) => {
                    let (root,sab) = val.into_root_value2(&name)?;
                    Ok(ArchivingItem::Item((name, root, sab)))
                },
                Err(e) => { Err(format!("filename {}, {}", name, e.to_string()))? }
            }
        },
        NameType::SystemName(_s) => {
            Err(format!("filename {} can't be used", file_name))?
        }
    }
}