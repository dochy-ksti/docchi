use std::fs::{ReadDir, DirEntry};
use crate::error::FsResult;
use crate::imp::common::list::file_data::FileData;
use crate::imp::common::list::find_next_dir::find_next_dir;

pub(crate) struct ListFilesIterator{
    ite : Option<InnerIterator>
}

struct InnerIterator {
    proj_dir : ReadDir,
    dir : ReadDir,
    hex : u128,
}

impl ListFilesIterator {
    pub(crate) fn new(proj_dir : ReadDir, dir : ReadDir, hex : u128) -> ListFilesIterator {
        let ite = InnerIterator { proj_dir, dir, hex };
        ListFilesIterator{ ite : Some(ite) }
    }

    pub(crate) fn create_empty() -> ListFilesIterator{
        ListFilesIterator{ ite : None }
    }
}

impl Iterator for ListFilesIterator{
    type Item=FsResult<FileData>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ite) = self.ite.as_mut(){
            ite.next()
        } else{
            None
        }
    }
}

impl InnerIterator {
    pub fn next(&mut self) -> Option<FsResult<FileData>> {
        loop {
            match find_next_file(&mut self.dir) {
                Ok(file_opt) => {
                    if let Some(file) = file_opt {
                        //found
                        return Some(FileData::from(self.hex, &file));
                    }
                    //Couldn't find a file. Go to the next directory.
                },
                Err(e) => {
                    //aborted
                    return Some(Err(e));
                }
            }

            let (dir,hex) = match find_next_dir(&mut self.proj_dir) {
                Ok(file_opt) => {
                    if let Some((dir, hex)) = file_opt {
                        //found the next directory
                        (dir,hex)
                    } else {
                        //end successfully
                        return None;
                    }
                },
                Err(e) => {
                    //aborted
                    return Some(Err(e));
                }
            };
            self.dir = dir;
            self.hex = hex;
        }

    }
}


fn find_next_file(file_iter : &mut ReadDir) -> FsResult<Option<DirEntry>>{
    if let Some(file) = file_iter.next(){
        let file = file?;
        return Ok(Some(file));
    } else{
        return Ok(None);
    }
}