use crate::error::FsResult;
use std::env::temp_dir;
use crate::test_simple_history::simple_diff::sd_data::SdData;
use rand::Rng;
use crate::test_simple_history::simple_diff::sd_cache::SdCache;
use crate::imp::history::algo::history_options::{HistoryOptionsBuilder, CumulativeOptionsBuilder};
use crate::imp::history::fs::load::load;
use crate::imp::history::file_hist::create_file_history::create_file_history;
use crate::imp::history::fs::next::_next;
use crate::test_simple_history::show_dir_contents_history::show_history_dir;
use crate::imp::history::fs::derive::_derive;

///ロード後にセーブしてもおかしくならないかチェック 主に目視で
//#[test]
fn test_load_and_save() -> FsResult<()> {
    let dir = temp_dir();
    let mut rng = rand::thread_rng();
    let dir_name = format!("test_load_and_save_{}",rng.gen_range(0..100_000_000));
    let dir = dir.join(&dir_name);
    //std::filesys::remove_dir(&dir).ok();
    std::fs::create_dir(&dir).ok();

    let op = HistoryOptionsBuilder::new()
        .max_phase(4)
        .update_phase_0(true)
        .cumulative(Some(CumulativeOptionsBuilder::new()
                             .limit_nth(Some(1))
                             .limit_count(Some(100))))
        .build()?;

    let mut data : SdData = SdData::new(None);
    let mut cache = SdCache::new(None);
    for _rep in 0..5{
        let n = rng.gen_range(1..=3);

        for _ in 0..n {
            data.mutate_randomly();
        }

        _next(None, &data, &mut cache, &dir, &op)?;
        let history = create_file_history(&dir, op.max_phase(), op.is_cumulative())?;
        let loaded = load(&history.newest_file_path(&dir)?, &history, cache.create_root(), &mut cache, &op)?;
        assert_eq!(loaded, data)
    }

    let history = create_file_history(&dir, op.max_phase(), op.is_cumulative())?;
    let files= history.list_files();
    let prev = files[files.len() - 2];
    assert_eq!(prev.phase(), 3);

    let mut data = load(&dir.join(prev.calc_filename()), &history, cache.create_root(), &mut cache, &op)?;
    _derive(None, &data, &mut cache, &dir, prev, &op)?;

    for _ in 0..30{
        let n = rng.gen_range(1..=3);

        for _ in 0..n {
            data.mutate_randomly();
        }

        _next(None, &data, &mut cache, &dir, &op)?;
        let history = create_file_history(&dir, op.max_phase(), op.is_cumulative())?;
        let loaded = load(&history.newest_file_path(&dir)?, &history, cache.create_root(), &mut cache, &op)?;
        assert_eq!(loaded, data)
    }

    //ちゃんとCumulativeOptionで設定したとおりに動いてるか頑張って確認してもいいが、
    //難しすぎて見合わない気がするので目視で適当に見る
    show_history_dir(&dir)?;

    Ok(())
}