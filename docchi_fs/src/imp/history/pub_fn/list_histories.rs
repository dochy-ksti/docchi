
use crate::history::{FileHistory, HistoryInfo};
use crate::common::get_hash_times;
use crate::error::FsResult;
use crate::imp::history::file_hist::file_histories::FileHistories;


pub fn list_histories(info : &HistoryInfo) -> FsResult<FileHistories>{
    let history_dir = info.history_dir();
    let hash_times = get_hash_times(history_dir)?;
    let opt = info.options();

    let mut vec : Vec<(u128, FileHistory)> = vec![];
    for (hash, _time) in hash_times{
        if let Ok(history) = FileHistory::create(history_dir, hash, opt.max_phase(), opt.is_cumulative()){
            vec.push((hash, history));
        }
    }

    Ok(FileHistories::new(vec))
}