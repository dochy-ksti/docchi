// use docchi_core::intf::RootObjectPtr;
// use docchi_core::structs::Qv;
// use tempfile::tempdir;
// use crate::error::FsResult;
// use crate::imp::common::current_src::CurrentSrc;
// use crate::history::{HistoryOptions, HistoryOptionsBuilder, list_histories, CumulativeOptionsBuilder, load_history_file_data, save_history_file};
// use docchi_core::intf::root::set_int;
// use crate::test_fs::copy_dir_all::copy_dir_all;
// use docchi_core::json_dir_to_root;
// use crate::test_diff_history::show_dir_contents_diff_history::show_dir_contents_diff_history;
// use crate::imp::common::dochy_cache::DochyCache;
// use crate::imp::history::history_info::HistoryInfo;
//
// ///途中でソースを変えるテスト
// //#[test]
// fn test_diff_his() -> FsResult<()> {
//     let dir = tempdir()?;
//     let proj_dir_path = dir.path();
//     let src_dir_path = proj_dir_path.join("simple_src");
//     let current_src = CurrentSrc::SrcDir(src_dir_path.clone());
//
//     copy_dir_all("src/json_dir/simple", &src_dir_path)?;
//
//
//     let opt = HistoryOptions::from(HistoryOptionsBuilder {
//         max_phase: 1,
//         update_phase_0: true,
//         cumulative: Some(CumulativeOptionsBuilder {
//             limit_count: Some(3),
//             limit_nth: None,
//         }),
//         ..Default::default()
//     })?;
//
//     let info = HistoryInfo::create(history_dir, current_src, &opt)?;
//
//
//     let mut root = json_dir_to_root(&src_dir_path, false)?;
//     for i in 0..2 {
//         let p = RootObjectPtr::new(&mut root);
//         set_int(p, "int", Qv::Val(i));
//
//         save_history_file(&info, None, &root)?;
//         let histories = list_histories(proj_dir_path, &opt)?;
//
//         let newest = histories.get_newest_file_data()?;
//
//
//         let loaded = load_history_file_data(proj_dir_path,
//                                             &newest, &mut cache, &opt, false)?;
//         assert!(root.contents_eq(&loaded));
//     }
//
//     std::fs::copy("src/json_dir/simple_mod1/root.json5", &src_dir_path.join("root.json5"))?;
//     let histories = list_histories(proj_dir_path, &opt)?;
//     let newest = histories.get_newest_file_data()?;
//
//     let mut root = load_history_file_data(proj_dir_path, &newest, &mut cache, &opt, false)?;
//     let p = RootObjectPtr::new(&mut root);
//     let mut cache = DochyCache::new(current_src.clone())?;
//
//     for i in 0..15 {
//         set_int(p, "int", Qv::Val(i));
//
//         save_history_file(proj_dir_path, None, &root, &mut cache, &opt)?;
//         let histories = list_histories(proj_dir_path, &opt)?;
//
//         let newest = histories.get_newest_file_data()?;
//
//         let loaded = load_history_file_data(proj_dir_path,
//                                             &newest, &mut cache, &opt, false)?;
//         assert!(root.contents_eq(&loaded));
//
//         histories.remove_old_files(5, proj_dir_path)?;
//
//         for (hash, name, size) in show_dir_contents_diff_history(proj_dir_path, &opt)?{
//             dbg!("{} {} {}", hash, name, size);
//         }
//
//
//     }
//
//
//
//     Ok(())
// }