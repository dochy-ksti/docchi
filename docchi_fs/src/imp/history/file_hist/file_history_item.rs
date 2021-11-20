use std::collections::BTreeMap;
use crate::imp::history::file_name::file_name_props::FileNameProps;
use crate::imp::history::remove::btree_zipper::BTreeZipper;

/// Historyが正しければ、itemの下にchildrenがあるわけだが、誰かが勝手にファイルを削除したりした場合、
/// itemがないのにchildrenだけあるといった事態も起こりうる
#[derive(Debug)]
pub(crate) struct FileHistoryItem{
    children : BTreeMap<u32, FileHistoryItem>,
    items : BTreeMap<u32, FileNameProps>,
}

impl FileHistoryItem{
    pub(crate) fn new() -> FileHistoryItem{
        FileHistoryItem{
            children : BTreeMap::new(),
            items : BTreeMap::new(),
        }
    }

    pub(crate) fn children(&self) -> &BTreeMap<u32, FileHistoryItem>{
        &self.children
    }
    pub(crate) fn items(&self) -> &BTreeMap<u32, FileNameProps>{
        &self.items
    }

    pub(crate) fn insert_or_get_mut(&mut self, order : u32) -> &mut FileHistoryItem {
        if self.children.contains_key(&order) == false{
            let new_his = FileHistoryItem::new();
            self.children.insert(order, new_his);
        }

        self.children.get_mut(&order).unwrap()
    }

    pub(crate) fn insert_props(&mut self, index : u32, props : FileNameProps){
        self.items.insert(index, props);
    }

    pub(crate) fn newest_child(&self) -> Option<(&u32, &FileHistoryItem)>{
        self.children.iter().last()
    }

    pub(crate) fn newest_item(&self) -> Option<(&u32, &FileNameProps)>{
        self.items.iter().last()
    }

    pub(crate) fn get_newest_prop(&self) -> Option<&FileNameProps> {
        if let Some((&item_ind, prop)) = self.newest_item() {
            if let Some((&child_ind, newest_child)) = self.newest_child() {
                if child_ind < item_ind {
                    return Some(prop);
                } else {
                    newest_child.get_newest_prop()
                }
            } else {
                return Some(prop);
            }
        } else if let Some((&_child_ind, newest_child)) = self.newest_child() {
            newest_child.get_newest_prop()
        } else{
            return None;
        }
    }

    pub(crate) fn flatten<'a, 'b>(&'a self, vec : &'b mut Vec<&'a FileNameProps>){
        let zipper = BTreeZipper::new(self.items(), self.children());
        for (_ind, child,props) in zipper{
            if let Some(item) = props{
                vec.push(item);
            }
            if let Some(child) = child{
                child.flatten(vec);
            }
        }
    }

    pub(crate) fn get_props(&self, order : &[u32]) -> Option<&FileNameProps>{
        if let Some(ind) = order.get(0) {
            if order.len() == 1 {
                self.items.get(ind)
            } else {
                if let Some(child) = self.children.get(ind){
                    child.get_props(&order[1..])
                } else{
                    None
                }
            }
        } else{
            None
        }
    }

    // pub(crate) fn _get_item(&self, order : &[u32]) -> Option<&FileHistoryItem> {
    //     if order.len() == 0 {
    //         return Some(self);
    //     }
    //     let ind = order[0];
    //     if let Some(child) = self.children.get(&ind) {
    //         child.get_item(&order[1..])
    //     } else {
    //         None
    //     }
    // }
}