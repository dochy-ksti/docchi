use crate::test_simple_history::simple_diff::sd_data::SdData;
use crate::test_simple_history::simple_diff::sd_diff::{SdDiff, SdDiffItem};

pub(crate) fn create_sd_diff(from : &SdData, to : &SdData) -> SdDiff{
    let mut vec : Vec<SdDiffItem> = vec![];
    for i in 0..from.len(){
        let f = from.get(i).unwrap();
        if let Some(t) = to.get(i){
            if f != t{
                vec.push(SdDiffItem::new(i, t))
            }
        }
    }
    SdDiff::new(vec)
}