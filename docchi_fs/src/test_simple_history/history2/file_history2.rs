use crate::imp::history::file_name::file_name_props::FileNameProps;
use crate::error::FsResult;
use std::collections::BTreeMap;
use crate::test_simple_history::history2::create_file_history2::create_file_history2;
use crate::test_simple_history::history2::file_history_item2::FileHistoryItem2;
use std::path::Path;

#[derive(Debug)]
pub(crate) struct FileHistory2{
    ctls : BTreeMap<u32, FileHistoryItem2>
}

impl FileHistory2{
    pub(crate) fn new() -> FileHistory2{
        FileHistory2{ ctls : BTreeMap::new() }
    }

    pub(crate) fn create(dir_path : &Path, hint_max_phase : Option<usize>) -> FsResult<FileHistory2>{
        create_file_history2(dir_path, hint_max_phase)
    }

    pub(crate) fn ctls(&self) -> &BTreeMap<u32, FileHistoryItem2>{
        &self.ctls
    }


    pub(crate) fn flatten(&self) -> Vec<(String, u64)>{
        let mut vec = vec![];
        for (_,ctl) in &self.ctls{
            ctl.flatten(&mut vec);
        }
        vec
    }

    fn insert_or_get_mut(&mut self, control : u32) -> &mut FileHistoryItem2 {
        if self.ctls.contains_key(&control) == false{
            let new_his = FileHistoryItem2::new();
            self.ctls.insert(control, new_his);
        }

        self.ctls.get_mut(&control).unwrap()
    }

    pub(crate) fn add(&mut self, props : FileNameProps, size : u64){
        let mut his = self.insert_or_get_mut(props.control());
        for &order in &props.order()[0..(props.order().len()-1)]{
            his = his.insert_or_get_mut(order);
        }
        let index = props.order_last();
        his.insert_props(index, props, size);
    }

    pub(crate) fn get_ctl(&self, control : u32) -> Option<&FileHistoryItem2>{
        self.ctls.get(&control)
    }
}

// fn get_history<'a, 'b>(history : &'a FileHistoryItem, order : &'b [u32], cumulative : bool) -> Option<Vec<&'a FileNameProps>>{
//     let mut vec = Vec::new();
//     let mut history = history;
//
//     for order in &order[0..order.len() - 1] {
//         let item = history.items().get(order)?;
//         vec.push(item);
//         history = history.children().get(order)?;
//     }
//     let order_last = *order.last().unwrap();
//     if cumulative{
//         for (index,prop) in history.items().iter(){
//             if *index <= order_last {
//                 vec.push(prop)
//             }
//         }
//     } else{
//         vec.push(&history.items()[&order_last])
//     }
//
//     return Some(vec)
// }