use std::collections::BTreeMap;
use crate::history::FileNameProps;
use crate::imp::history::file_hist::file_history_item::FileHistoryItem;
use std::collections::btree_map::Iter;

/// indexが低い方から、アイテムをiterateしていくが、
/// 同indexに両アイテムがあれば Some Someが帰り、
/// 一方しかない場合一方はSome, 一方はNoneになる、
/// 両者なければ単にNoneが返り、iterate終了
pub(crate) struct BTreeZipper<'a>{
    props : Iter<'a, u32, FileNameProps>,
    his : Iter<'a, u32, FileHistoryItem>,

    cur_props : i64,
    cur_his : i64,

    r_props : Option<&'a FileNameProps>,
    r_his : Option<&'a FileHistoryItem>,
}

impl<'a> BTreeZipper<'a>{
    pub(crate) fn new(
        props : &'a BTreeMap<u32, FileNameProps>,
        his : &'a BTreeMap<u32, FileHistoryItem>) -> BTreeZipper<'a>{
        BTreeZipper{
            props : props.iter(),
            his : his.iter(),
            cur_props : -1,
            cur_his : -1,
            r_props : None,
            r_his : None,
        }
    }
}

impl<'a> Iterator for BTreeZipper<'a>{
    type Item = (u32, Option<&'a FileHistoryItem>, Option<&'a FileNameProps>);

    fn next(&mut self) -> Option<Self::Item> {
        let move_props;
        let move_his;

        if self.cur_props == self.cur_his{
            if self.cur_props == i64::MAX{
                return None;
            }
            move_props = true;
            move_his = true;
        } else if self.cur_props < self.cur_his{
            move_props = true;
            move_his = false;
        } else{
            move_props = false;
            move_his = true;
        }

        if move_props{
            if let Some((cur, props)) = self.props.next(){
                self.r_props = Some(props);
                self.cur_props = *cur as i64;
            } else{
                self.r_props = None;
                self.cur_props = i64::MAX;
            }
        }
        if move_his{
            if let Some((cur, his)) = self.his.next(){
                self.r_his = Some(his);
                self.cur_his = *cur as i64;
            } else{
                self.r_his = None;
                self.cur_his = i64::MAX;
            }
        }

        if self.cur_props == self.cur_his {
            if self.cur_props == i64::MAX {
                None
            } else {
                Some((self.cur_props as u32, self.r_his, self.r_props))
            }
        }
        else if self.cur_props < self.cur_his {
            Some((self.cur_props as u32, None, self.r_props))
        } else {
            Some((self.cur_his as u32, self.r_his, None))
        }
    }
}

