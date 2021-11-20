use std::collections::BTreeMap;
use crate::imp::history::file_name::file_name_props::FileNameProps;

#[derive(Debug)]
pub(crate) struct FileHistoryItem2{
    children : BTreeMap<u32, FileHistoryItem2>,
    items : BTreeMap<u32, (FileNameProps, u64)>,
}

impl FileHistoryItem2{
    pub(crate) fn new() -> FileHistoryItem2{
        FileHistoryItem2{
            children : BTreeMap::new(),
            items : BTreeMap::new(),
        }
    }

    pub(crate) fn children(&self) -> &BTreeMap<u32, FileHistoryItem2>{
        &self.children
    }
    pub(crate) fn items(&self) -> &BTreeMap<u32, (FileNameProps, u64)>{
        &self.items
    }

    pub(crate) fn insert_or_get_mut(&mut self, order : u32) -> &mut FileHistoryItem2 {
        if self.children.contains_key(&order) == false{
            let new_his = FileHistoryItem2::new();
            self.children.insert(order, new_his);
        }

        self.children.get_mut(&order).unwrap()
    }

    pub(crate) fn insert_props(&mut self, index : u32, props : FileNameProps, size : u64){
        self.items.insert(index, (props, size));
    }

    pub(crate) fn flatten(&self, vec : &mut Vec<(String, u64)>){
        for (a,(props, size)) in &self.items{
            vec.push((props.calc_filename(), *size));
            if self.children.contains_key(a){
                self.children[a].flatten(vec);
            }
        }
    }
}