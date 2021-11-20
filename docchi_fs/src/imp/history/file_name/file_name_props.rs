use crate::imp::history::file_name::calc_filename::calc_filename;
use crate::error::FsResult;
use crate::imp::history::file_name::analyze_file_name::analyze_file_name;

///Metadata history files' filename contains.
///The filename can be precisely restored from it.
#[derive(Debug, Clone, PartialEq)]
pub struct FileNameProps {
    control: u32,
    prev_ctl : u32,
    order : Vec<u32>,
    tag: Option<String>,
}

impl FileNameProps{

    /// Create item from metadata.
    pub fn new(control : u32, prev_ctl : u32, order : Vec<u32>, tag : Option<String>) -> FsResult<FileNameProps>{
        if order.len() == 0{
            Err("order.len() must be >= 1")?
        }
        Ok(FileNameProps{ control, prev_ctl, order, tag })
    }

    pub fn from(filename : &str) -> Option<FileNameProps>{
        analyze_file_name(filename, None)
    }

    ///how many times "start_new" is called or deriving has happened.
    pub fn control(&self) -> u32{ self.control }

    /// If this item is derived, "control" of the parent item,
    /// otherwise this item's "control"
    pub fn prev_ctl(&self) -> u32{ self.prev_ctl }


    /// numbers which describes how this item is derived.
    /// if the order is `[3,8,2]`, this item depends on items which has the order of `[3]` and `[3,8]`
    ///
    /// order.len >= 1
    pub fn order(&self) -> &[u32]{ &self.order }

    /// self.order.len() - 1
    pub fn phase(&self) -> usize{ self.order.len() - 1 }

    /// the last item of the order is excluded
    /// panics if the order.len() == 0
    pub fn order_base(&self) -> &[u32]{
        let len = self.order.len() - 1;
        &self.order[0..len]
    }
    pub(crate) fn order_last(&self) -> u32{ *self.order.last().unwrap() }

    /// string data users can append in this item's filename.
    pub fn tag(&self) -> Option<&str>{ self.tag.as_ref().map(|s| s.as_str()) }

    /// calculate the filename from the metadata this item contains.
    pub fn calc_filename(&self) -> String{
        calc_filename(self.tag(), self.control(),
                      if self.control() == self.prev_ctl(){ None } else{ Some(self.prev_ctl()) }
                      ,self.order())
    }


    pub(crate) fn _create_next_phase_props(&self, ctl : u32, tag : Option<String>, next_phase : usize) -> Option<FileNameProps> {
        let mut order = self.order.clone();
        if order.len() < next_phase{
            order.push(0);
        } else{
            let order_last = self.order_last();
            *order.last_mut()? = order_last + 1;
        }

        return Some(FileNameProps::new(ctl, self.control(), order, tag).ok()?)
    }

}