use crate::sample_test::sample_code::rpg2_accessor::{ItemsMItem, ItemsEnum};

pub trait ItemUtil{
    fn price(&self) -> i64;
}

impl ItemUtil for ItemsMItem{
    fn price(&self) -> i64 {
        match self.get_enum(){
            ItemsEnum::Herb(h) =>{ h.price() }
            ItemsEnum::Sword(s) =>{ s.price() }
        }
    }
}
