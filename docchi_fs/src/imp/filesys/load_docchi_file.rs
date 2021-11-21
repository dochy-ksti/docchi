use std::path::Path;
use crate::error::FsResult;
use docchi_core::structs::RootObject;
use std::fs::File;
use docchi_diff::apply_diff;
use docchi_core::{adjust_versions, archive_file_to_root};
use crate::imp::common::path::reserved_filename::ARCHIVE_DEFAULT_NAME;
use crate::common::hash::folder_name_to_hash;
use crate::imp::filesys::save_dir_info::SaveDirInfo;

pub fn load_docchi_file<P : AsRef<Path>>(file_path : P,
                                         current_save_dir_info : &SaveDirInfo,
                                         validation : bool) -> FsResult<RootObject>{
    let path = file_path.as_ref();
    let dir_path = path.parent().ok_or("file_path's file must be in a folder which contains src.archive file.")?;
    let folder_name = dir_path.file_name().ok_or_else(||format!("the path doesn't have a folder name {:?} ", dir_path))?;
    let hash = folder_name_to_hash(folder_name).ok_or_else(||format!("folder name doesn't contain hash value {:?}", folder_name))?;


    let current_src_hash = current_save_dir_info.hash();
    let src_root = current_save_dir_info.clone_src_root();

    let mut root = if current_src_hash == hash{
        src_root.clone()
    } else {
        //archiveに関しては、カレントしかキャッシュしないことにする
        let archive_path = dir_path.join(ARCHIVE_DEFAULT_NAME);
        archive_file_to_root(archive_path, validation)?
    };

    let mut file = File::open(file_path)?;
    apply_diff(&mut root, &mut file)?;

    if current_src_hash != hash{
        Ok(adjust_versions(src_root, root, validation)?)
    } else{
        Ok(root)
    }
}