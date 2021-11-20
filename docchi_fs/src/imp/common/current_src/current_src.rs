use std::path::{PathBuf, Path};
use crate::error::FsResult;
use std::io::Write;

/// We always have an archive of json src files which corresponds to save files.
/// if there's no change in json src files, we don't need current src.
/// if the src is changed, we need the archived and the current src
/// because we need to update the save data and make it compatible with the current version.
/// The current src can be specified as an archive or a directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CurrentSrc{
    SrcDir(PathBuf),
    ArchiveFile(PathBuf),
}

impl CurrentSrc{
    pub fn from_src_dir<P : AsRef<Path>>(src_dir : P) -> CurrentSrc{
        CurrentSrc::SrcDir(PathBuf::from(src_dir.as_ref()))
    }

    pub fn from_archive_file<P : AsRef<Path>>(archive_file_path : P) -> CurrentSrc{
        CurrentSrc::ArchiveFile(PathBuf::from(archive_file_path.as_ref()))
    }

    pub fn create_archive<W : Write>(&self, write : &mut W) -> FsResult<()>{
        match &self{
            CurrentSrc::SrcDir(src_dir) => {
                docchi_core::archive_src_dir(src_dir, write)?;
                Ok(())
            },
            CurrentSrc::ArchiveFile(path) =>{
                let mut file = std::fs::File::open(path)?;
                std::io::copy(&mut file, write)?;
                Ok(())
            }
        }
    }
}