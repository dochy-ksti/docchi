use std::collections::{BTreeMap, HashMap};
use crate::imp::history::file_name::file_name_props::FileNameProps;
use crate::imp::history::file_hist::file_history_item::FileHistoryItem;
use crate::imp::history::remove::history_remover_item::HistoryRemoverItem;
use crate::imp::history::remove::btree_zipper::BTreeZipper;

pub(crate) fn composite_remover<'a>(props : &'a BTreeMap<u32, FileNameProps>,
                                his : &'a BTreeMap<u32, FileHistoryItem>,
                                cur_phase : usize,
                                max_phase : usize,
                                cumulative_option : bool)
        -> HashMap<u32, HistoryRemoverItem<'a>>{
    let len = his.len().max(props.len());
    let mut r : HashMap<u32, HistoryRemoverItem> = HashMap::with_capacity(len);
    let zipper = BTreeZipper::new(props, his);
    let cumulative = cumulative_option && cur_phase == max_phase;

    for (ind, his, props) in zipper{
        if let Some(his) = his {
            if cur_phase <= max_phase {
                r.insert(ind, HistoryRemoverItem::from(his, props, cur_phase + 1, max_phase, cumulative_option));
            }
        } else {
            r.insert(ind, HistoryRemoverItem::new(cumulative, props, HashMap::new()));
        }
    }
    r
}