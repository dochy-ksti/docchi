use std::path::Path;
use crate::error::FsResult;
use crate::imp::common::list::list_files_iterator::ListFilesIterator;
use crate::imp::common::list::find_next_dir::find_next_dir;
use crate::imp::common::list::file_data::FileData;
use crate::imp::common::path::reserved_filename::is_reserved_filename;


pub(crate) fn list_files<P : AsRef<Path>>(proj_dir: P) -> FsResult<Vec<FileData>>{
    let save_dir = proj_dir.as_ref();
    let iter = iterate_files(save_dir)?;
    let mut vec : Vec<FileData> = vec![];

    for item in iter{
        if let Ok(data) = item {
            if is_reserved_filename(data.name()){
                continue;
            }
            vec.push(data);
        }
    }

    vec.sort_by_key(|a| a.modified());
    Ok(vec)
}

fn iterate_files(save_dir: &Path) -> FsResult<ListFilesIterator>{
    let mut dir_iter = std::fs::read_dir(save_dir)?;
    if let Some((file_iter, hex)) = find_next_dir(&mut dir_iter)? {
        return Ok(ListFilesIterator::new(dir_iter, file_iter, hex));
    } else{
        return Ok(ListFilesIterator::create_empty());
    }

}