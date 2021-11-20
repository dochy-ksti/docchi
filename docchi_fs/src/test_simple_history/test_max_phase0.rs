use crate::error::FsResult;
use std::env::temp_dir;
use crate::test_simple_history::simple_diff::sd_data::SdData;
use crate::imp::history::fs::next::_next;
use rand::Rng;
use crate::test_simple_history::simple_diff::sd_cache::SdCache;
use crate::imp::history::algo::history_options::{HistoryOptionsBuilder};
use crate::imp::history::fs::load::load;
use crate::imp::history::file_hist::create_file_history::create_file_history;

///max_phase 0 みたいな無意味な設定でも正しく動くかチェック
//#[test]
fn test_max_phase0() -> FsResult<()> {
    let dir = temp_dir();
    let mut rng = rand::thread_rng();
    let dir_name = format!("test_simple_diff_{}",rng.gen_range(0..100_000_000));
    let dir = dir.join(&dir_name);
    //std::filesys::remove_dir(&dir).ok();
    std::fs::create_dir(&dir).ok();

    let op = HistoryOptionsBuilder::new()
        .max_phase(0)
        .update_phase_0(true)
        .cumulative(None)
        .build()?;

    let mut data : SdData = SdData::new(Some(200));
    let mut cache = SdCache::new(Some(200));
    let repeat = 100;
    for _rep in 0..repeat {
        let n = rng.gen_range(1..=3);

        for _ in 0..n {
            data.mutate_randomly();
        }


        _next(None, &data, &mut cache, &dir, &op)?;

        let history = create_file_history(&dir, op.max_phase(), op.is_cumulative())?;
        //show_history_dir(&dir);
        let newest_path = history.newest_file_path(&dir)?;
        //dbg!(&newest_path);
        let loaded = load(newest_path, &history, cache.create_root(), &mut cache, &op)?;

        assert_eq!(loaded, data)
    }

    Ok(())
}