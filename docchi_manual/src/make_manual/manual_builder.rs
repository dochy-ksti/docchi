use std::path::Path;
use docchi::error::DpResult;
use crate::make_manual::make_page::make_page;
use crate::make_manual::write_page::{write_page, write_index_page};
use crate::make_manual::make_index_page::make_index_page;
use docchi::core::structs::NullOr;

pub(crate) struct ManualBuilder {
    vec : Vec<ManualBuilderItem>
}

pub(crate) struct ManualBuilderItem {
    title : String,
    src : NullOr<String>,
}

impl ManualBuilderItem {
    pub(crate) fn new(title : String, src : NullOr<String>) -> ManualBuilderItem {
        ManualBuilderItem { title, src }
    }
    pub(crate) fn title(&self) -> &str{ &self.title }
    pub(crate) fn src(&self) -> NullOr<&str>{ self.src.as_ref().map(|s| s.as_str()) }
}

impl ManualBuilder {
    pub(crate) fn new() -> ManualBuilder { ManualBuilder { vec : vec![] } }
    pub(crate) fn add(&mut self, title : String, src : NullOr<String>){
        self.vec.push(ManualBuilderItem::new(title, src))
    }

    pub(crate) fn build<P : AsRef<Path>>(&self, manual_dir : P) -> DpResult<()>{
        let manual_dir = manual_dir.as_ref();

        let vec = &self.vec;
        for i in 0..vec.len(){
            let item = vec.get(i).unwrap();
            if let NullOr::Val(src) = &item.src {
                let prev = get_src(vec, i.overflowing_sub(1).0);
                let next = get_src(vec, i + 1);

                let page = make_page(prev, next, item.title(), src)?;
                write_page(src, &page, manual_dir)?;
            }
        }
        let index_page = make_index_page(vec)?;
        write_index_page(&index_page, manual_dir)?;
        Ok(())
    }
}

fn get_src(vec : &[ManualBuilderItem], index : usize) -> NullOr<&str>{
    match vec.get(index) {
        Some(v) => v.src.as_ref().map(|s| s.as_str()),
        None => NullOr::Null
    }
}