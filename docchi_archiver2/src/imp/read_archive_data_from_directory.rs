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
    let root_path = path.as_ref();
    get_paths_from_dir(root_path, "", opt, &mut btree)?;
    let mut archiver = Archiver::new(converter);
    for rel_path in btree {
        let data = std::fs::read(root_path.join(&rel_path))?;
        archiver.archive(rel_path, data);
    }
    let hoge = archiver.finish();
    return hoge;
}

fn get_paths_from_dir<P1 : AsRef<Path>, P2 : AsRef<Path>>(root_path : P1, rel_path : P2, opt : &ArchiveOptions, btree : &mut BTreeSet<String>) -> ArcResult<()>{
    let root_path = root_path.as_ref();
    let rel_path = rel_path.as_ref();
    let current_path = root_path.join(rel_path);
    let dirs = std::fs::read_dir(current_path)?;

    for dir in dirs {
        let de = dir?;

        let meta = de.metadata()?;
        if meta.is_file() {
            let path = de.path();

            let ext = path.extension().map_or("", |e| e.to_str().unwrap_or(""));
            if opt.is_archived(ext){
                if let Some(filename) = path.file_name() {
                    let rel = rel_path.join(filename);
                    btree.insert(rel.to_string_lossy().to_string());
                }
            }
        } else if meta.is_dir(){
            if opt.archive_subfolders(){
                let rel = rel_path.join(de.file_name());
                get_paths_from_dir(root_path, &rel,  opt, btree)?;

            }
        }
    }
    Ok(())
}