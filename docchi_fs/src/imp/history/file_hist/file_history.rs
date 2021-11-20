use crate::imp::history::file_name::file_name_props::FileNameProps;
use crate::imp::history::file_hist::file_history_item::FileHistoryItem;
use crate::imp::history::file_hist::create_file_history::create_file_history;
use std::path::{Path, PathBuf};
use crate::error::FsResult;
use std::collections::BTreeMap;
use crate::imp::history::remove::history_remover::HistoryRemover;
use crate::imp::common::path::hash_dir_path::hash_dir_path;

/// Represents every history file in a hash directory
#[derive(Debug)]
pub struct FileHistory{
    ctls : BTreeMap<u32, FileHistoryItem>,
    max_phase : usize,
    cumulative : bool,
}

impl FileHistory{
    pub(crate) fn new(max_phase : usize, cumulative : bool) -> FileHistory{
        FileHistory{ ctls : BTreeMap::new(), max_phase, cumulative }
    }

    pub fn max_phase(&self) -> usize{ self.max_phase }
    pub fn cumulative(&self) -> bool{ self.cumulative }

    pub fn create<P : AsRef<Path>>(history_dir: P, hash : u128, max_phase : usize, cumulative : bool) -> FsResult<FileHistory>{
        let dir = hash_dir_path(history_dir.as_ref(), hash);
        create_file_history(&dir, max_phase, cumulative)
    }

    /// show every file in chronological order
    pub fn list_files(&self) -> Vec<&FileNameProps>{
        let mut vec = vec![];
        for (_,ctl) in &self.ctls{
            ctl.flatten(&mut vec);
        }
        vec
    }

    ///returns old files which can be removed.
    pub fn get_removable_old_items(&self, keep_latest : usize) -> Vec<&FileNameProps>{
        let files = self.list_files();
        let remover = HistoryRemover::from(self);
        let num_remove = files.len().saturating_sub(keep_latest);
        for item in files.iter().skip(num_remove) {
            remover.keep(*item);
        }
        let props = remover.get_removable_props();
        props
    }

    /// Remove old files. Files will be deleted in this method,
    /// and this history will be inconsistent with saved files, so the history will be consumed.
    ///
    /// Returns file paths which is failed to remove if any.
    pub fn remove_old_files<P:AsRef<Path>>(self, keep_latest : usize, history_hash_dir: P) -> Vec<PathBuf>{
        let removables = self.get_removable_old_items(keep_latest);
        unsafe{ Self::remove_files(removables.iter().map(|a| *a), history_hash_dir) }
    }

    /// remove files.
    /// History files have dependencies, so arbitrary removing can make files inconsistent.
    ///
    /// If the "filenames" are created from "get_removable_old_items", this function itself will be safe,
    /// but removing files make existing FileHistory inconsistent with saved files.
    ///
    /// Returns paths which is failed to remove
    pub unsafe fn remove_files<'a, P : AsRef<Path>>(filenames : impl Iterator<Item=&'a FileNameProps>, history_hash_dir: P) -> Vec<PathBuf>{
        let dir_path = history_hash_dir.as_ref();
        let mut r : Vec<PathBuf> = vec![];
        for filename in filenames {
            let file_path = dir_path.join(filename.calc_filename());
            match std::fs::remove_file(&file_path){
                Ok(_) =>{},
                Err(_) =>{ r.push(file_path) }
            }
        }
        r
    }

    /// get the newest file path in this hash_dir
    pub fn newest_file_path(&self, history_hash_dir: &Path) -> Option<PathBuf>{
        self.get_newest_prop().map(|props|{
            let filename = props.calc_filename();
            history_hash_dir.join(filename)
        })
    }

    pub(crate) fn ctls(&self) -> &BTreeMap<u32, FileHistoryItem>{ &self.ctls }

    fn newest_ctl(&self) -> Option<&FileHistoryItem>{
        self.ctls.iter().last().map(|(_,item)| item)
    }

    pub(crate) fn get_newest_prop(&self) -> Option<&FileNameProps>{
        let his = if let Some(control_his) = self.newest_ctl(){
            control_his
        } else{
            return None;
        };

        his.get_newest_prop()
    }



    fn insert_or_get_mut(&mut self, control : u32) -> &mut FileHistoryItem {
        if self.ctls.contains_key(&control) == false{
            let new_his = FileHistoryItem::new();
            self.ctls.insert(control, new_his);
        }

        self.ctls.get_mut(&control).unwrap()
    }

    pub(crate) fn add(&mut self, props : FileNameProps){
        let mut his = self.insert_or_get_mut(props.control());
        for &order in &props.order()[0..(props.order().len()-1)]{
            his = his.insert_or_get_mut(order);
        }
        let index = props.order_last();
        his.insert_props(index, props);
    }

    // pub(crate) fn get_ctl(&self, control : u32) -> Option<&FileHistoryItem>{
    //     self.ctls.get(&control)
    // }

    pub fn get_props(&self, ctl : u32, order : &[u32]) -> Option<&FileNameProps>{
        if let Some(h) = self.ctls.get(&ctl){
            h.get_props(order)
        } else{
            None
        }
    }

    // pub(crate) fn _get_item(&self, ctl : u32, order : &[u32]) -> Option<&FileHistoryItem>{
    //     if let Some(h) = self.ctls.get(&ctl){
    //         h._get_item(order)
    //     } else{
    //         None
    //     }
    // }

    pub fn get_parent(&self, props : &FileNameProps) -> Option<&FileNameProps>{
        self.get_props(props.prev_ctl(), props.order_base())
    }

    // pub(crate) fn get_ancestor_ctl(&self, props : &FileNameProps, phase : usize) -> FsResult<u32>{
    //     let v = create_ancestors_rev(self, props, self.max_phase, self.cumulative)?;
    //     let len = v.len();
    //     Ok(v.get(len - 1 - phase)?.control())
    // }
}

