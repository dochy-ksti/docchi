use crate::error::CoreResult;
use std::str::from_utf8;
use std::path::Path;
use docchi_json5::JVal;
use crate::imp::json_to_rust::json_name::{json_name_re, NameType, json_simple_name};
use crate::imp::json_to_rust::{json_item_str_to_rust, json_root_to_rust};
use crate::imp::json_to_rust::json_obj_to_rust::json_obj_to_rust;
use crate::imp::json_to_rust::names::Names;
use crate::imp::structs::docchi_archive::ArchivingItem;

enum PathType{
    Direct,
    Indirect,
    Invalid
}
fn path_type(path : &Path) -> PathType{
    let path = if let Some(path) = path.parent(){ path } else{
        //空文字列や、セパレータのみの場合、parentはない。pathは相対パスなので、空文字列ということになるが、
        //アーカイバの性質上ないものはアーカイブしないので、ここにはこないはず
        //(ファイル名のないファイル、などというものがあれば別だが、さすがにそんなものをパスで表すことは出来ない
        return PathType::Invalid;
    };
    let path = if let Some(path) = path.parent(){ path } else{
        //2回のparentでNoneが返る場合、ルート直下のファイル
        return PathType::Direct;
    };
    if let Some(_) = path.parent(){
        //2段階以上のファイルは扱わない
        return PathType::Invalid;
    } else{
        //ディレクトリを挟んでファイルがある。この場合、TableのItemという仕様
        return PathType::Indirect;
    };
}

pub(crate) fn json_file_to_rust(path: &str, dat: &[u8]) -> CoreResult<ArchivingItem> {
    let s = from_utf8(dat)?;
    let path = Path::new(path);

    match path_type(path){
        PathType::Direct => direct(path, s),
        PathType::Indirect => indirect(path, s),
        PathType::Invalid => Err(format!("{:?} : invalid path", path))?,
    }
}

fn direct(path :&Path, s : &str) -> CoreResult<ArchivingItem>{
    let file_stem = path.file_stem().unwrap().to_string_lossy();
    let file_stem = file_stem.as_ref();

    if file_stem == "root" {
        return Ok(ArchivingItem::Root(json_root_to_rust(s)?));
    }
    let name_type = json_name_re(file_stem)?;
    match name_type {
        NameType::Name(name, var_type) => {
            match json_item_str_to_rust(s, &name, var_type.clone()) {
                Ok(val) => {
                    let (root, sab) = val.into_root_value2(&name)?;
                    Ok(ArchivingItem::Item((name, root, sab)))
                },
                Err(e) => { Err(format!("filename {}, {}", name, e.to_string()))? }
            }
        },
        NameType::SystemName(_s) => {
            Err(format!("filename {} can't be used", file_stem))?
        }
    }
}

fn indirect(path : &Path, s : &str) -> CoreResult<ArchivingItem>{
    let parent_name = path.parent().unwrap().file_name().unwrap().to_string_lossy().to_string();
    let file_stem = path.file_stem().unwrap().to_string_lossy().to_string();
    if json_simple_name(&file_stem).is_none(){
        Err(format!("filename {} is not a valid name", file_stem))?
    }
    if let JVal::Map(map, span) = docchi_json5::from_str(s)?{
        let obj = json_obj_to_rust(&map, false, &span, &Names::new(&parent_name))?;
        let item = obj.into_list_item()?;
        return Ok(ArchivingItem::TableItem((parent_name, file_stem, item)));
    } else{
        Err(format!("{}.{}: Invalid Json5", parent_name, file_stem))?
    }

}
