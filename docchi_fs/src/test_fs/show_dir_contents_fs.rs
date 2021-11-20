use std::path::{Path, PathBuf};
use crate::error::FsResult;
use std::fs::{read_dir};

pub fn show_dir_contents_fs<P : AsRef<Path>>(path : P) -> FsResult<Vec<PathBuf>>{
    let mut r : Vec<PathBuf> = vec![];
    collect_dir_contents(path.as_ref(), &mut r)?;
    Ok(r)
}

fn collect_dir_contents(path : &Path, r : &mut Vec<PathBuf>) -> FsResult<()>{
    let dir = read_dir(path)?;

    let mut dirs : Vec<PathBuf> = vec![];

    for item in dir{
        let entry = item?;
        r.push(entry.path());
        if entry.file_type()?.is_dir(){
            dirs.push(entry.path());
        }
    }

    for dir_path in dirs{
        collect_dir_contents(&dir_path, r)?;
    }
    Ok(())
}