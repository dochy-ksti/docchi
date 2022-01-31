use crate::error::CoreResult;
use crate::structs::RootObject;
use std::io::Read;
use crate::imp::json_to_rust::validation::validate_root::validate_root;
use crate::imp::json_to_rust::roots::archive_data_to_root::archive_data_to_root_with_hash;
use std::path::Path;
use std::fs::File;
use crate::imp::json_to_rust::roots::json_file_to_rust::json_file_to_rust;

pub fn read_archive_to_root_with_hash<R : Read>(r : &mut R, validation : bool) -> CoreResult<(RootObject, u128)>{
    let data = docchi_archiver2::read_archive(json_file_to_rust, r)?;
    let (root, hash) = archive_data_to_root_with_hash(data)?;
    if validation{
        validate_root(&root, false)?;
    }
    Ok((root, hash))
}

pub fn read_archive_to_root<R : Read>(r : &mut R, validation : bool) -> CoreResult<RootObject>{
    read_archive_to_root_with_hash(r, validation).map(|(root,_)| root)
}

pub fn archive_file_to_root_with_hash<P : AsRef<Path>>(archive : P, validation : bool) -> CoreResult<(RootObject, u128)>{
    let mut file = File::open(archive)?;
    read_archive_to_root_with_hash(&mut file, validation)
}

pub fn archive_file_to_root<P : AsRef<Path>>(archive : P, validation : bool) -> CoreResult<RootObject>{
    let (root, _hash) = archive_file_to_root_with_hash(archive, validation)?;
    Ok(root)
}