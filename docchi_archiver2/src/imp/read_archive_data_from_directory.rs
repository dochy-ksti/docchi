use std::path::Path;
use crate::{ArcResult, ArchiveOptions};
use crate::imp::structs::archiver::Archiver;
use std::collections::BTreeSet;
use crate::imp::structs::archive_data::ArchiveData;

pub fn read_archive_data_from_directory<
    P : AsRef<Path>,
    T : Send + 'static>(path : P,
                        opt : &ArchiveOptions,
                        converter : impl Fn(&str, &[u8]) -> T + Send + Sync + 'static) -> ArcResult<ArchiveData<T>>{
    let mut btree : BTreeSet<String> = BTreeSet::new();
    get_paths_from_dir(path, opt, &mut btree)?;
    let mut archiver = Archiver::new(converter);
    for path in btree {
        let data = std::fs::read(&path)?;
        archiver.archive(path, data);
    }
    let hoge = archiver.finish();
    return hoge;
}

fn get_paths_from_dir<P : AsRef<Path>>(current_path : P, opt : &ArchiveOptions, btree : &mut BTreeSet<String>) -> ArcResult<()>{
    let dirs = std::fs::read_dir(current_path)?;

    for dir in dirs {
        let de = dir?;

        let meta = de.metadata()?;
        if meta.is_file() {
            let path = de.path();
            let ext = path.extension().map_or("", |e| e.to_str().unwrap_or(""));
            if opt.is_archived(ext){
                btree.insert(path.to_string_lossy().to_string());
            }
        } else if meta.is_dir(){
            if opt.archive_subfolders(){
                get_paths_from_dir(de.path(), opt, btree)?;
            }
        }
    }
    Ok(())
}