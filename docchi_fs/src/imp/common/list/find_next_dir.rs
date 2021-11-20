use std::fs::{ReadDir, read_dir};
use crate::error::FsResult;
use crate::imp::common::path::hash::folder_name_to_hash;

pub(crate) fn find_next_dir(dir_iter : &mut ReadDir) -> FsResult<Option<(ReadDir, u128)>>{
    loop {
        if let Some(entry) = dir_iter.next() {
            let entry = entry?;
            if entry.file_type()?.is_dir(){
                //名前がhexであるdirectoryだけ探して返す
                if let Some(hex) = folder_name_to_hash(&entry.file_name()){
                    let d = read_dir(&entry.path())?;
                    return Ok(Some((d, hex)));
                }
            }
        } else{
            return Ok(None);
        }
    }
}
