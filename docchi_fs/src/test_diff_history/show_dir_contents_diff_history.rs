use crate::error::FsResult;
use std::fs::{File};
use crate::history::{list_histories, HistoryInfo};

pub fn show_dir_contents_diff_history(info : &HistoryInfo) -> FsResult<Vec<(u128, String, u64)>>{
    let proj_dir = info.history_dir();
    let histories = list_histories(info)?;
    let mut r : Vec<(u128, String, u64)> = vec![];
    for item in histories.list_files(){
        let name = item.props().calc_filename();
        let file = File::open(item.calc_path(proj_dir))?;
        r.push((item.hash(), name, file.metadata()?.len()))
    }
    Ok(r)
}