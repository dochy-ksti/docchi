use docchi_core::structs::RootObject;
use std::path::PathBuf;
use crate::error::FsResult;
use crate::imp::history::diff_and_cache::open_diff_file_without_metadata::open_diff_file_without_metadata;
use docchi_diff::{apply_diff, RootDiffR};
use std::sync::mpsc::{Sender, Receiver};
use crate::imp::history::diff_and_cache::docchi_diff::DocchiDiff;
use std::collections::HashMap;

pub(crate) fn apply_items_st<F : FnMut(&RootObject)>(mut root : RootObject,
                                          paths : &[PathBuf],
                                          mut func : F) -> FsResult<RootObject> {

    for path in paths{
        let mut file = open_diff_file_without_metadata(&path)?;
        apply_diff(&mut root, &mut file)?;
        func(&root);
    }
    Ok(root)
}

///マルチスレッド化したので早くなることが期待される
pub(crate) fn apply_items_mt<F : FnMut(&RootObject)>(mut root : RootObject,
                                                     paths : Vec<PathBuf>,
                                                     mut func : F) -> FsResult<RootObject> {


    let (sender, receiver): (Sender<(usize,RootDiffR)>, Receiver<(usize,RootDiffR)>) = std::sync::mpsc::channel();
    let meta_table_arc = root.meta_table_arc();
    let paths_len = paths.len();

    std::thread::spawn(move ||{
        for (i,path) in paths.iter().enumerate() {
            let mut file = open_diff_file_without_metadata(path).unwrap();
            let v = DocchiDiff::read_value(&mut file).unwrap();
            let sender = sender.clone();
            let meta_table_arc = meta_table_arc.clone();
            rayon::spawn_fifo(move ||{
                let va = v.prepare(meta_table_arc.as_ref()).unwrap();
                sender.send((i, va)).unwrap();
            });
        }
    });
    let mut map : HashMap<usize, RootDiffR> = HashMap::new();
    let mut current_index = 0;
    for t_i in 0..paths_len {
        let (i,va) = receiver.recv().or(Err(format!("preparing recv {} failed", t_i)))?;
        map.insert(i, va);
        apply_several_items(&mut root, &mut current_index, &mut map, &mut func)?;
    }
    Ok(root)
}

fn apply_several_items<F : FnMut(&RootObject)>(root : &mut RootObject,
                                               current_index: &mut usize,
                                               map: &mut HashMap<usize, RootDiffR>,
                                               func : &mut F) -> FsResult<()>{
    loop {
        if let Some(diff) = map.remove(current_index) {
            docchi_diff::apply_root_diff_r(root, diff)?;
            *current_index += 1;
            func(root);
        } else{
            break;
        }
    }
    Ok(())
}