use docchi::error::DpResult;
use docchi::fs::common::{CurrentSrc};
use std::path::{Path};
use docchi::fs::filesys::{SaveDirInfo, save_dochy_file_nb, load_dochy_file, list_dochy_files};
use crate::fs_test::test_save_async::test_save_async_accessor::RootIntf;
use std::time::Duration;
//use std::lazy::Lazy;
use parking_lot::FairMutex;
use once_cell::sync::Lazy;

static VEC_LAZY: Lazy<FairMutex<Vec<String>>> = Lazy::new(||{
    FairMutex::new(Vec::new())
});

#[test]
fn test_save_async() -> DpResult<()> {
    let root_dir = Path::new("src/fs_test/test_save_async");
    let save_dir = root_dir.join("save_dir");
    std::fs::create_dir(&save_dir).ok();
    let info = SaveDirInfo::create(&save_dir,
                                   CurrentSrc::from_src_dir("src/fs_test/test_save_async/src_dir"))?;

    let root = info.clone_src_root();
    let mut root = RootIntf::new(root);
    let max = 10;

    //let mut r : Vec<String> = Vec::new();
    for i in 0..max{
        let info_copy = info.clone();
        root.set_data0(i);
        save_dochy_file_nb(&info,
                           &format!("file{}.dochy", i),
                           root.root_obj_ref(), true,
                           move |_r|{
                                  let mut v = VEC_LAZY.lock();
                                  v.push(format!("{} finished num_threads {}",i, info_copy.queued_threads()));
                              });
    }

    loop{
        std::thread::sleep(Duration::from_millis(100));
        if info.queued_threads() == 0{
            break;
        }
    }

    let v = VEC_LAZY.lock();
    let hoge : &Vec<String> = &v;
    println!("{:?}", hoge);

    let ds = list_dochy_files(&save_dir)?;
    let last = ds.last().unwrap();
    let ld = load_dochy_file(last.calc_path(&save_dir), &info, true)?;
    let ld = RootIntf::new(ld);
    println!("data0 {}", ld.data0());
    Ok(())
}