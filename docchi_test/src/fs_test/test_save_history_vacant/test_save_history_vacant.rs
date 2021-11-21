use docchi::error::DpResult;
use docchi::fs::common::{CurrentSrc};
use std::path::{Path};

use std::time::Duration;
use once_cell::sync::Lazy;

use docchi::fs::history::{HistoryInfo, list_histories, load_history_file, save_history_file_nb_if_vacant};

use std::sync::Mutex as Mutex;
use crate::fs_test::test_save_history_vacant::test_save_history_vacant_accessor::RootIntf;
//use parking_lot::FairMutex as Mutex;

static VEC_LAZY: Lazy<Mutex<Vec<String>>> = Lazy::new(||{
    Mutex::new(Vec::new())
});

#[test]
fn test_save_history_vacant() -> DpResult<()> {
    let root_dir = Path::new("src/fs_test/test_save_history_vacant");
    let history_dir = root_dir.join("history_dir");
    let src_dir =root_dir.join("src_dir");

    std::fs::remove_dir_all(&history_dir).ok();
    std::fs::create_dir(&history_dir).ok();

    let info = HistoryInfo::create(&history_dir,
                                   CurrentSrc::from_src_dir(&src_dir), ())?;

    let root = info.clone_src_root();
    let mut root = RootIntf::new(root);
    let max = 10;

    for i in 0..max{

        root.set_data0(i);
        if save_history_file_nb_if_vacant(&info,
                                None,
                                root.root_obj_ref(), move |_r|{

                let mut v = VEC_LAZY.lock().unwrap();
                v.push(format!("callback {}", i));
            }) == false{
        }
    }

    loop{
        std::thread::sleep(Duration::from_millis(100));

        if info.peekable().queued() == 0{
            break;
        }
    }

    let v = VEC_LAZY.lock().unwrap();
    println!("{:?}", &v);

    let hiss = list_histories(&info)?;
    for d in hiss.list_files(){
        let loaded = load_history_file(&info, d.props(), d.history(), true)?;
        let l = RootIntf::new(loaded);
        println!("{:?} {:?}", l.data0(), d.props().calc_filename());
    }

    Ok(())
}