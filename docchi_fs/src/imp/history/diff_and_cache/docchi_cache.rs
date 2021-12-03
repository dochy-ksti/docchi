use std::path::{PathBuf, Path};
use crate::error::FsResult;
use docchi_core::structs::{RootObject};
use std::collections::{BTreeMap};
use crate::imp::common::apply_items::{apply_items_st, apply_items_mt};
use crate::history::HistoryOptions;
use crate::common::{ CurrentSrc};
use crate::imp::common::archive::get_archive_path::get_archive_path2;
use crate::imp::common::current_src::current_src_map::get_current_src_info;
use docchi_core::archive_file_to_root;


pub(crate) struct DocchiCache {
    //current_src : CurrentSrc,
    src_root : RootObject,
    hash : u128,
    ///ほんとはVecでいいんだよなあ
    pub(crate) phase_cache: BTreeMap<usize, (PathBuf, RootObject)>,
}

impl DocchiCache {


    pub(crate) fn get_or_create_hash_root<P:AsRef<Path>>(&self, history_dir : P, hash : u128) -> FsResult<RootObject>{
        if self.hash() != hash {
            Ok(archive_file_to_root(get_archive_path2(history_dir, hash), false)?)
        } else{
            Ok(self.clone_src_root())
        }
    }

    pub(crate) fn new(current_src : CurrentSrc) -> FsResult<DocchiCache>{
        let info = get_current_src_info(current_src)?;

        Ok(DocchiCache {
            //current_src : info.current_src().clone(),
            src_root : info.clone_src_root(),
            hash : info.hash(),
            phase_cache : BTreeMap::new(),
        })
    }

    //pub fn current_src(&self) -> &CurrentSrc{ &self.current_src }
    pub(crate) fn hash(&self) -> u128{ self.hash }

    pub(crate) fn clone_src_root(&self) -> RootObject{
        self.src_root.clone()
    }

    pub(crate) fn apply_items_for_save(&mut self, paths: Vec<PathBuf>, op : &HistoryOptions) -> FsResult<RootObject> {
        let first_len = paths.len();
        let (root,paths) = get_cached_item(self.clone_src_root(), self, paths, op.max_phase())?;
        let num_cached = first_len - paths.len();

        //キャッシュを削除しない場合、phase0から古いデータ→新しいデータという関係性が壊れる
        //Binaryは値の比較をせず、Arcのポインタ比較だけで同値性を判断するが、
        // 大きいphaseのデータを比較するときに、小さいphaseのキャッシュがないとphase0からデータを作り直すので、
        // 同値であっても違うArcなので違うと判定され、巨大なデータになってしまう
        // それを避けるためキャッシュは古いデータ→新しいデータの関係性を保つべきだろう
        remove_upper_phase_cache(&mut self.phase_cache, num_cached);
        let mut current_index = num_cached;

        let cache_func = |r : &RootObject| {
            if current_index < op.max_phase() {
                let path = &paths[current_index - num_cached];
                self.phase_cache.insert(current_index, (path.to_path_buf(), r.clone()));
                current_index += 1;
            }
        };
        if op.mt_save() == false {
            //こっちがデフォルト
            //セーブは非同期で実行されるので、時間がかかってもかまうことはない、という発想
            //マルチスレッドにすることで、他の処理が遅くなる方が問題が大きいのではないかと思う
            apply_items_st(root, &paths, cache_func)
        } else{
            apply_items_mt(root, paths.to_vec(),   cache_func)
        }
    }

    pub(crate) fn apply_items_for_load(&mut self, load_root : RootObject, paths: Vec<PathBuf>, op : &HistoryOptions) -> FsResult<RootObject> {
        let (root,paths) = get_cached_item(load_root,self, paths, op.max_phase())?;

        if op.mt_load() {
            //こっちがデフォルト
            //ロードではキャッシュを行わず、全力でただ開く。ロードが終わるまで処理が進まないことが想定されている
            //ロードで中途半端にキャッシュを作ると、キャッシュの古→新 関係が壊れるので、全部作るか、全部やめるかの2択
            //なのでロード時にキャッシュは作らんほうがよいだろう
            apply_items_mt(root, paths, |_| {})
        } else{
            apply_items_st(root, &paths, |_| {})
        }
    }

    pub(crate) fn set_cache(&mut self, path : PathBuf, item: RootObject, phase: usize) {
        remove_upper_phase_cache(&mut self.phase_cache, phase);
        self.phase_cache.insert(phase, (path, item));
    }
}

/// removes phase <= index
fn remove_upper_phase_cache(cache : &mut BTreeMap<usize, (PathBuf, RootObject)>, phase : usize){
    loop{
        let index = if let Some((index, _)) = cache.iter().last(){ *index }
        else{
            break;
        };
        if phase <= index{
            cache.remove(&index);
        } else{
            break;
        }
    }
}

fn get_cached_item(root : RootObject, cache : &mut DocchiCache, paths: Vec<PathBuf>, max_phase : usize) -> FsResult<(RootObject, Vec<PathBuf>)> {
    if let Some(index) = get_phase_cache(&cache.phase_cache, &paths, max_phase){
        if index == max_phase{
            let (_,root) = cache.phase_cache.get(&index).unwrap();
            return Ok((root.clone(), Vec::new()));
        } else{
            let root = {
                let (_, root) = cache.phase_cache.get(&index).unwrap();
                root.clone()
            };
            let ans = paths.into_iter().skip(index+1).collect();
            return Ok((root, ans));
        }
    } else{
        Ok((root, paths))
    }
}

fn get_phase_cache(cache : &BTreeMap<usize, (PathBuf, RootObject)>, vec : &Vec<PathBuf>, max_phase : usize) -> Option<usize>{
    let (index,(path, _)) = cache.iter().last()?;
    let last_path = vec.iter().last()?;
    if path == last_path {
        //多分このパターンしかないと思うんだよね。
        //ただの計算のショートカットだから完全網羅しなくてもいいはず・・・
        return Some(*index);
    }

    for i in (0..max_phase).rev(){
        if let Some((c,_)) = cache.get(&i){
            if let Some(p) = vec.get(i){
                if c == p{ return Some(i); }
            }
        }
    }
    return None;
}
