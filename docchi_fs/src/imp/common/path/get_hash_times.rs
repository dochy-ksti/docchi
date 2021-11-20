use std::time::SystemTime;
use std::path::Path;
use crate::error::FsResult;
use crate::common::hash::folder_name_to_hash;
use crate::imp::common::path::created_time_file::{ from_time_dat};
use std::fs::{File, Metadata};
use crate::imp::common::path::reserved_filename::CREATED_TIME_FILE_NAME;

///Sorted by modified time
pub fn get_hash_times<P : AsRef<Path>>(proj_dir : P) -> FsResult<Vec<(u128, SystemTime)>>{
    //let proj_dir = proj_dir.as_ref();
    let dir = std::fs::read_dir(proj_dir)?;
    let mut r : Vec<(u128, SystemTime)> = vec![];

    for entry in dir{
        let entry = entry?;
        let meta = entry.metadata()?;
        if meta.is_dir(){
            //0-9A-Fで構成された32文字のフォルダは、ハッシュであると考えられる。
            //そうじゃない場合困ったことになるので、そういう変なものをproj_dirに作らないように・・・
            if let Some(hash) = folder_name_to_hash(&entry.file_name()){
                let file_path = entry.path().join(CREATED_TIME_FILE_NAME);
                let time = get_folder_time(file_path, &meta);
                r.push((hash, time));
            }
        }
    }

    r.sort_by_key(|(_hash, time)| *time);
    Ok(r)
}

fn get_folder_time<P : AsRef<Path>>(path : P, hash_dir_meta : &Metadata) -> SystemTime {
    if let Ok(mut file) = File::open(path) {
        if let Ok(time) = from_time_dat(&mut file){
            return time;
        }
    }
    if let Ok(time) = hash_dir_meta.created() {
        //metadata の created time は フォルダをコピーしたりなんかすると失われてしまう可能性があるので、
        //信頼性が低い。なのでdatファイルを埋めているが、datが見つからない場合は使う
        return time;
    }
    //なにもみつからない・・・？？ そんなことないと思うけど、いちおう
    SystemTime::UNIX_EPOCH
}