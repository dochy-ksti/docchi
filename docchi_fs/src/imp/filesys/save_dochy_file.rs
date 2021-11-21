use std::path::{PathBuf};
use std::fs::File;
use crate::error::FsResult;
use std::io::Write;
use docchi_core::structs::RootObject;
use crate::imp::filesys::save_dir_info::SaveDirInfo;
use crate::imp::filesys::save_cache_map::{get_mutex};
use crate::imp::common::path::prepare_hash_dir::prepare_hash_dir;


/// saves a Docchi file.
pub fn save_dochy_file(info : &SaveDirInfo,
                       file_name : &str,
                       root: &RootObject,
                       overwrite : bool) -> FsResult<PathBuf>{
    let mutex = get_mutex(info.save_dir()).unwrap();
    let info = mutex.cache();
    let save_dir = info.save_dir();
    let hash_dir = prepare_hash_dir(save_dir, info.current_src(), info.hash())?;
    let src_root = info.clone_src_root();

    let file_path = hash_dir.join(file_name);
    if file_path.exists() && overwrite == false {
        Err("file already exists")?
    }
    let diff = docchi_diff::get_diff(&src_root, &root)?;
    let mut file = File::create(&file_path)?;
    file.write_all(&diff)?;
    Ok(file_path)
}

/// non-blocking version. spawns 1 OS thread.
/// Saves clone of the RootObject, and cloning is very fast because RootObject consists of Arcs.
/// Actual copying occurs only when the RootObject is modified before the saving is finished,
/// and only the parts to be modified are copied using Arc::make_mut.
pub fn save_dochy_file_nb<
    F : FnOnce(FsResult<PathBuf>) + Send + 'static>(info : &SaveDirInfo,
                                                    file_name : &str,
                                                    root: &RootObject,
                                                    overwrite : bool,
                                                    callback : F) {
    let info = info.clone();
    let file_name = file_name.to_string();
    let root = root.clone();
    std::thread::spawn(move ||{
        match save_dochy_file(&info, &file_name, &root, overwrite){
            Ok(p) =>{
                callback(Ok(p.to_path_buf()));
            },
            Err(e) =>{
                callback(Err(e));
            }
        }
    });
}

