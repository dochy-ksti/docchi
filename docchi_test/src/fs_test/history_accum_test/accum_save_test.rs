use docchi::error::DpResult;
use docchi::fs::common::{CurrentSrc, hash_dir_path};
use std::path::{Path};
use docchi::fs::history::{save_history_file, list_histories, load_history_file, CurrentRootObjInfo, HistoryInfo};
use docchi::core::structs::RootObject;
use crate::fs_test::history_accum_test::hello_history_accessor::RootIntf;
use rand::Rng;

///10メンバのうち1メンバを上書きするので90%はそのままだが、
/// 各メンバ(Binary)がジワジワ伸びていくので全体のファイルサイズは大きくなっていく
/// そんな状態が正しくHistoryとして記録できているか、目視とload and compareで確認
#[test]
fn accum_save_test() -> DpResult<()> {
    let info = HistoryInfo::create("src/fs_test/history_accum_test/history_dir",
    CurrentSrc::from_src_dir("src/fs_test/history_accum_test/src_dir"), ())?;

    let root = info.clone_src_root();
    let history_dir = info.history_dir();

    std::fs::create_dir(history_dir).ok();
    let his = list_histories(&info)?;
    his.remove_old_files(0, history_dir)?;

    let mut root = RootIntf::new(root);

    for _counter in 0..60{
        mutate_root(&mut root);
        let saved = save_history_file(&info, None, root.root_obj_ref())?;
        let loaded = RootIntf::new(load_newest_file(&info)?);
        //mutate_root(&mut loaded);
        assert!(root.root_obj_ref().contents_eq(loaded.root_obj_ref()));
        //ロードするとロードしたオブジェクトでセーブしないとHistoryが構成できない
        //ここはインチキして、セーブした時の状態に戻す
        docchi::fs::history::set_current_root_obj_info(
            &info, Some(CurrentRootObjInfo::new(root.root_obj_ref().id(), saved)));
    }

    print_file_data(hash_dir_path(history_dir, info.hash()))?;

    let loaded = RootIntf::new(load_newest_file(&info)?);
    assert_eq!(loaded.data0(), root.data0());
    dbg!(root.data0());
    Ok(())
}

fn mutate_root(r : &mut RootIntf){
    let m = match rand::thread_rng().gen_range(0..=9){
        0 => r.data0_mut(),
        1 => r.data1_mut(),
        2 => r.data2_mut(),
        3 => r.data3_mut(),
        4 => r.data4_mut(),
        5 => r.data5_mut(),
        6 => r.data6_mut(),
        7 => r.data7_mut(),
        8 => r.data8_mut(),
        9 => r.data9_mut(),
        _ => unreachable!(),
    };

    //IntArrayの場合は、乱数0を引いて変化なしの場合かなり小さなファイルサイズになる。
    //Binaryの場合、変化なしかどうかを比較して確かめないので、どうしても大きくなる
    let l = rand::thread_rng().gen_range(0..=5);
    for _ in 0..l{
        let k = rand::thread_rng().gen_range(0..=100);
        m.push(k);
    }
}

fn print_file_data<P : AsRef<Path>>(path : P) -> DpResult<()>{
    for file in std::fs::read_dir(path)?{
        let file = file?;
        println!("{} size {}", file.file_name().to_str().unwrap(), file.metadata().unwrap().len());
    }
    Ok(())
}


fn load_newest_file(info : &HistoryInfo) -> DpResult<RootObject>{
    let his = list_histories(info)?;
    let d = his.get_newest_file_data().unwrap();

    Ok(load_history_file(info, d.props(), d.history(),false)?)
}