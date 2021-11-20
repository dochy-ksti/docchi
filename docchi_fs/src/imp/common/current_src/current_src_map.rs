use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::common::{CurrentSrc};
use crate::error::FsResult;
use docchi_core::structs::RootObject;
use docchi_core::{json_dir_to_root_with_hash, archive_file_to_root_with_hash};
use crate::imp::common::current_src::current_src_info::CurrentSrcInfo;
use std::sync::Mutex;


static MAP : Lazy<Mutex<HashMap<CurrentSrc, Box<CurrentSrcInfo>>>> = Lazy::new(||{
    Mutex::new(HashMap::new())
});

pub(crate) fn get_current_src_info(current_src : CurrentSrc) -> FsResult<CurrentSrcInfo>{
    let mut map = MAP.lock().unwrap();
    if let Some(item) = map.get(&current_src){
        return Ok(item.as_ref().clone())
    }
    let (root, hash) = create_root_and_hash(&current_src)?;
    let info = CurrentSrcInfo::new(current_src.clone(), root, hash);
    map.insert(current_src, Box::new(info.clone()));
    Ok(info)
}

fn create_root_and_hash(current_src : &CurrentSrc) -> FsResult<(RootObject, u128)>{
    match current_src{
        CurrentSrc::SrcDir(src_dir) => {
        let (root,hash) = json_dir_to_root_with_hash(src_dir, false)?;
            Ok((root, hash))
        },
        CurrentSrc::ArchiveFile(path) =>{
            Ok(archive_file_to_root_with_hash(path,false)?)
        }
    }
}
