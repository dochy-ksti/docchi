use std::collections::HashMap;
use crate::imp::history::file_hist::file_history::FileHistory;
use crate::imp::history::remove::history_remover_item::{HistoryRemoverItem, RemoveCueItem};
use crate::imp::history::file_name::file_name_props::FileNameProps;
use crate::imp::history::file_hist::file_history_item::FileHistoryItem;
use crate::imp::history::remove::composite_remover::composite_remover;
use std::path::{Path, PathBuf};

/// Determines if files are safe to be removed
pub struct HistoryRemover<'a>{
    ctls : HashMap<u32, HistoryRemoverCtlItem<'a>>
}

pub(crate) struct HistoryRemoverCtlItem<'a>{
    items: HashMap<u32, HistoryRemoverItem<'a>>
}

impl<'a> HistoryRemover<'a>{

    /// Set flags to keep the file and its ancestors
    pub fn keep(&self, props : &FileNameProps) -> Option<()> {
        let mut cue = RemoveCueItem::from(props);
        loop {
            let ctl_item = self.ctls.get(&cue.ctl())?;
            let item = ctl_item.get_item_from_cue(cue.order(), cue.order_last())?;
            if let Some(r) = item.keep() {
                cue = r;
            } else {
                break;
            }
        }
        Some(())
    }



    /// find all the files which can be removed
    pub fn get_removable_props(&self) -> Vec<&'a FileNameProps>{
        let mut vec : Vec<&FileNameProps> = vec![];
        for (_,ctl) in &self.ctls{
            ctl.get_removable_props(&mut vec);
        }
        vec
    }

    /// remove all the files without the files to be kept and their dependencies
    /// returns the paths which is failed to remove
    pub fn remove<P : AsRef<Path>>(self, history_hash_dir : P) -> Vec<PathBuf>{
        let removables = self.get_removable_props();
        unsafe{
            FileHistory::remove_files(removables.into_iter(), history_hash_dir)
        }
    }

    pub fn from(history : &'a FileHistory) -> HistoryRemover<'a>{
        let src_ctls = history.ctls();
        let mut r_ctls : HashMap<u32, HistoryRemoverCtlItem> = HashMap::with_capacity(src_ctls.len());
        for (index, ctl) in src_ctls{
            r_ctls.insert(*index, HistoryRemoverCtlItem::from(ctl, history.max_phase(), history.cumulative()));
        }
        HistoryRemover{ ctls : r_ctls }
    }
}

impl<'a> HistoryRemoverCtlItem<'a>{
    pub(crate) fn from(ctl : &'a FileHistoryItem, max_phase : usize, cumulative_option : bool) -> HistoryRemoverCtlItem<'a>{
        let r = composite_remover(ctl.items(),ctl.children(),
                                  0, max_phase, cumulative_option);
        HistoryRemoverCtlItem{ items : r }
    }

    pub(crate) fn get_item_from_cue(&self, cue_order : &[u32], cue_order_last : Option<u32>) -> Option<&'a HistoryRemoverItem>{
        let mut item = self.items.get(cue_order.get(0)?)?;

        for ind in &cue_order[1..]{
            item = item.children().get(ind)?;
        }

        if let Some(ind) = cue_order_last{
            item = item.children().get(&ind)?;
        }
        Some(item)
    }

    pub(crate) fn get_removable_props(&self, r : &mut Vec<&'a FileNameProps>){
        for (_,ctl) in &self.items {
            ctl.get_removable_props(r);
        }
    }
}
