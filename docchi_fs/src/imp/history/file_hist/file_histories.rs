use crate::imp::history::file_hist::file_history::FileHistory;
use crate::imp::history::file_hist::history_file_data::HistoryFileData;
use std::path::Path;
use crate::imp::common::path::hash_dir_path::hash_dir_path;
use std::ops::Range;
use crate::common::remove_hash_dir;
use crate::error::FsResult;

/// Represents every file history in every hash directory of a project
#[derive(Debug)]
pub struct FileHistories{
    vec : Vec<(u128, FileHistory)>,
}

impl FileHistories{
    pub(crate) fn new(vec : Vec<(u128, FileHistory)>) -> FileHistories{ FileHistories{ vec } }

    /// list every HistoryFileData chronologically
    pub fn list_files(&self) -> Vec<HistoryFileData>{
        self.vec.iter().flat_map(|(hash, his)|{
            //let hash = *hash;
            his.list_files().into_iter()
                .map(move |props| HistoryFileData::new(*hash, his, props))
        }).collect()
    }

    /// gets the newest HisotryFileData
    pub fn get_newest_file_data(&self) -> Option<HistoryFileData>{
        self.vec.last().and_then(|(hash, his)|
                                     his.get_newest_prop().map(|prop|
                                     HistoryFileData::new(*hash, his, prop)))
    }

    pub fn iter(&self) -> impl Iterator<Item=&(u128, FileHistory)>{
        self.vec.iter()
    }

    /// remove old files other than latest n files. This function consumes history data
    pub fn remove_old_files<P : AsRef<Path>>(self, keep_latest : usize, history_dir : P) -> FsResult<()>{
        let mut s = self;
        unsafe{ s.remove_old_files_us(keep_latest, history_dir) }
    }

    /// remove old files other than latest n files. This function doesn't consume history data,
    /// and the data will be inconsistent with the actual files
    pub unsafe fn remove_old_files_us<P : AsRef<Path>>(&mut self, keep_latest : usize, history_dir : P) -> FsResult<()>{
        let len = self.vec.len();
        let history_dir = history_dir.as_ref();
        if len == 0{ return Ok(()); }
        if keep_latest == 0{
            // 最新ファイルに付随する制御ファイルがあって面倒なので、ディレクトリごと全部削除してしまう
            remove_hash_dirs(&mut self.vec, 0..len,history_dir)?;
            return Ok(());
        }
        let (hash, his) = self.vec.last().unwrap();
        let hash_dir_path = hash_dir_path(history_dir, *hash);
        if keep_latest < his.list_files().len(){
            // 最新ハッシュの全てをkeepしない。
            // つまり過去ハッシュフォルダは全削除し、
            // 最新ハッシュの一部だけを残す
            let len = self.vec.len();
            if 2 <= len {
                remove_hash_dirs(&mut self.vec, 0..len - 1, history_dir)?;
            }
            let (_,last) = self.vec.remove(0);
            last.remove_old_files(keep_latest, hash_dir_path);
            return Ok(());
        }
        // こっちは最新ハッシュの全てをkeepする場合
        // この場合、全削除されるハッシュフォルダがあるなら、それだけ削除する
        let mut sum = 0;
        let mut ind = self.vec.len() - 1;

        //hash_dir単位でのいい加減な削除。
        while 1 <= ind{
            let index = ind;
            ind -= 1;

            let (_, his) = self.vec.get(index).unwrap();
            let len = his.list_files().len();
            sum += len;
            if keep_latest < sum{
                remove_hash_dirs(&mut self.vec,0..index, history_dir.as_ref())?;
                return Ok(());
            }
        }
        Ok(())
    }
}

fn remove_hash_dirs(vec : &mut Vec<(u128, FileHistory)>, range : Range<usize>, history_dir : &Path) -> FsResult<()>{
    //drainとか使ってファイルとデータを一致させようと頑張ってはいるが、最終的に技術的難題にぶつかって完全な同期を諦めているのであまり意味がない
    for (hash, _) in vec.drain(range){
        remove_hash_dir(history_dir, hash)?;
    }
    Ok(())
}