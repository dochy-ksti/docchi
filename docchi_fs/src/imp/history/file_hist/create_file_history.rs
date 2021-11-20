use std::path::{Path};
use crate::error::FsResult;
use crate::imp::history::file_hist::file_history::FileHistory;
use crate::imp::history::file_name::analyze_file_name::analyze_file_name;


pub(crate) fn create_file_history(history_hash_dir: &Path, max_phase : usize, cumulative : bool) -> FsResult<FileHistory>{
    let dir = std::fs::read_dir(history_hash_dir)?;
    let mut history = FileHistory::new(max_phase, cumulative);
    for entry in dir {
        let entry = entry?;
        let filename = entry.path().file_name()
            .ok_or_else(|| format!("The filename couldn't be found {:?}", entry.path()))?.to_string_lossy().to_string();
        if let Some(props) = analyze_file_name(&filename, Some(max_phase)) {
            history.add(props)
        } else{
        }
    }
    Ok(history)
}