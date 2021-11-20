use crate::error::FsResult;
use std::env::temp_dir;
use crate::test_simple_history::simple_diff::sd_data::SdData;
use crate::imp::history::fs::next::_next;
use rand::Rng;
use crate::test_simple_history::simple_diff::sd_cache::SdCache;
use crate::imp::history::algo::history_options::{HistoryOptionsBuilder, CumulativeOptionsBuilder};
use crate::test_simple_history::history2::file_history2::FileHistory2;
use crate::test_simple_history::history2::file_history_item2::FileHistoryItem2;
use std::collections::BTreeMap;
use crate::imp::history::file_name::file_name_props::FileNameProps;

///nthに合わせてcumulative phaseが伸びていくか調べる。目視で
//#[test]
fn test_cumulative_limit_nth() -> FsResult<()> {
    let dir = temp_dir();
    let mut rng = rand::thread_rng();
    //なんかtemp_dir消えてないことあるんよね
    let dir_name = format!("test_simple_diff_{}",rng.gen_range(0..100_000_000));
    let dir = dir.join(&dir_name);
    std::fs::create_dir(&dir).ok();

    let op = HistoryOptionsBuilder::new()
        .max_phase(2)
        .update_phase_0(true)
        .cumulative(Some(CumulativeOptionsBuilder::new()
                        .limit_nth(Some(2))
                        .limit_count(None)))
        .build()?;

    let mut data : SdData = SdData::new(None);
    let mut cache = SdCache::new(None);
    let repeat = 100;
    for _rep in 0..repeat{
        data.mutate(1)?;
        _next(None, &data, &mut cache, &dir, &op)?;
    }

    let history = FileHistory2::create(&dir, Some(op.max_phase()))?;


    for (_,ctl) in history.ctls(){
        let mut vec : Vec<bool> = vec![];
        find(&SizeList::new(0), ctl,2,
             &(|items, list| check(items, list, 2)),
             &mut vec);
        assert!(vec.iter().all(|b| *b))
    }

    let vec = history.flatten();
    for (name,size) in &vec{
        println!("{} {}", name, *size);
    }

    Ok(())
}

struct SizeList<'a>{
    pub next : Option<&'a SizeList<'a>>,
    pub size : u64,
}

impl<'a> SizeList<'a>{
    fn new(size : u64) -> SizeList<'a>{
        SizeList{ size, next : None }
    }
    fn append(&'a self, size :u64) -> SizeList<'a>{
        SizeList{ next : Some(self), size }
    }
    fn current(&self) -> u64{ self.size }
    fn len(&self) -> usize{
        let mut len = 1;
        let mut l = self;
        loop{
            if let Some(next) = l.next{
                len += 1;
                l = next;
            } else{
                return len;
            }
        }
    }
    fn to_vec(&self) -> Vec<u64>{
        let mut vec : Vec<u64> = vec![];
        let mut l = self;
        loop{
            vec.push(l.size);
            if let Some(next) = l.next{
                l = next;
            } else{
                return vec;
            }
        }
    }
}

fn find(sizes : &SizeList, child : &FileHistoryItem2, depth_goal : usize,
         check : &impl Fn(&BTreeMap<u32, (FileNameProps, u64)>, &SizeList) -> bool,
         result : &mut Vec<bool>){
    if depth_goal < sizes.len() {
        result.push(check(child.items(), sizes));
        return;
    }
    for (index, (_item, size)) in child.items(){
        if let Some(child) = child.children().get(index){
            find(&sizes.append(*size), child, depth_goal, check, result);
        }
    }
}

fn check(items : &BTreeMap<u32, (FileNameProps, u64)>, sizes : &SizeList, nth : usize) -> bool{
    let mut vec = sizes.to_vec();
    vec.sort();
    let total = vec[vec.len() - nth];

    let mut sum = 0;
    let mut len = 0;
    for (_index, (_props, size)) in items{
        len += 1;
        sum += *size;
        if total < sum{
            if len != items.len() && len != 1 {
                return false;
            }
        }
    }
    return true;
}