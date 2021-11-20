use crate::imp::history::file_name::file_name_props::FileNameProps;
use std::path::{Path, PathBuf};
use crate::imp::common::path::hash_dir_path::hash_dir_path;
use crate::history::FileHistory;

/// Data of a history file.
#[derive(Debug, Clone, Copy)]
pub struct HistoryFileData<'a>{
    hash : u128,
    history : &'a FileHistory,
    props : &'a FileNameProps,
}

impl<'a> HistoryFileData<'a>{
    /// constructs the struct
    pub fn new(hash : u128, history : &'a FileHistory, props : &'a FileNameProps) -> HistoryFileData<'a>{ HistoryFileData{ hash, history, props } }

    /// hash value of the source file this history file derives from
    pub fn hash(&self) -> u128{ self.hash }

    /// reference to the parent directory's data of this file
    pub fn history(&self) -> &'a FileHistory{ &self.history }

    /// filename and the metadata encoded in it
    pub fn props(&self) -> &'a FileNameProps{ &self.props }

    /// calculates the path of this file
    pub fn calc_path<P : AsRef<Path>>(&self, history_dir : P) -> PathBuf{
        let hash_dir = hash_dir_path(history_dir.as_ref(), self.hash);
        hash_dir.join(self.props.calc_filename())
    }
}