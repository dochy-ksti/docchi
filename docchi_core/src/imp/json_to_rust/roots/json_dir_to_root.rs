

use crate::error::CoreResult;

use crate::imp::structs::root_obj::RootObject;
use std::path::Path;
use docchi_archiver2::read_archive_data_from_directory;
use crate::JSON_ARC_OPT;
use crate::imp::json_to_rust::validation::validate_root::validate_root;
use crate::imp::json_to_rust::roots::archive_data_to_root::archive_data_to_root_with_hash;
use crate::imp::json_to_rust::roots::json_file_to_rust::json_file_to_rust;
use crate::imp::structs::docchi_archive::DocchiArchive;

/// Converts Docchi source files to RootObject
/// Does extra checks when validation=true
pub fn json_dir_to_root_with_hash<P : AsRef<Path>>(dir_path : P, validation : bool) -> CoreResult<(RootObject, u128)> {
    let archive = json_dir_to_archive(dir_path)?;
    let (root, hash) = archive_data_to_root_with_hash(archive.data)?;
    if validation{
        validate_root(&root, false)?;
    }
    return Ok((root, hash));
}

pub fn json_dir_to_root<P : AsRef<Path>>(dir_path : P, validation : bool) -> CoreResult<RootObject> {
    json_dir_to_root_with_hash(dir_path, validation).map(|(root,_)| root)
}

pub(crate) fn json_dir_to_archive<P : AsRef<Path>>(dir_path : P) -> CoreResult<DocchiArchive>{
    let data = read_archive_data_from_directory(dir_path, &JSON_ARC_OPT, json_file_to_rust)?;
    for (_name,item) in data.btree(){
        if let Err(e) = item.converted_data(){
            Err(format!("{}", e))?
        }
    }
    Ok(DocchiArchive::new(data))
}