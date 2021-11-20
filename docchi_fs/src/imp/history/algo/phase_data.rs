use docchi_compaction::kval_enum::KVal;
use crate::error::{FsResult, OptToErr};
use docchi_compaction::basic_compaction;
use crate::imp::history::algo::phase_data_item::PhaseDataItem;
use with_capacity_safe::vec_with_capacity_safe;


pub(crate) struct PhaseData{
    items : Vec<PhaseDataItem>,
}

impl PhaseData{
    pub(crate) fn new(size : u64) -> PhaseData{
        PhaseData{ items : vec![PhaseDataItem::new(size)] }
    }

    pub(crate) fn push(&mut self, size : u64){
        for item in self.items.iter_mut(){
            item.add(size);
        }
        self.items.push(PhaseDataItem::new(size));
    }

    /// pop は 親 phase への シフトを意味しており、pushはそのphaseへの追加を表している
    ///
    /// 平行に追加したい時は 1回 popしてpushすればいい。
    ///
    /// この処理によって、最新のPhase-Aから見たファイルサイズ分布、 Bから見たファイルサイズ分布・・・が保持&更新される
    pub(crate) fn pop_and_push(&mut self, next_phase : usize, size_to_push : u64) {
        while next_phase < self.items.len(){
            self.items.pop();
        }

        self.push(size_to_push);
    }

    // fn iter_mut(&mut self) -> impl Iterator<Item=&mut PhaseDataItem>{
    //     self.items.iter_mut()
    // }

    pub(crate) fn len(&self) -> usize{
        self.items.len()
    }
    pub(crate) fn get(&self, climb : usize) -> Option<&PhaseDataItem>{
        self.items.get(self.items.len() - 1 - climb)
    }

    pub(crate) fn encode(&self) -> Vec<KVal>{
        let mut vec : Vec<KVal> = Vec::with_capacity(self.items.len() * 3 + 1);
        vec.push(basic_compaction::comp_int(self.items.len() as i64));
        for item in &self.items{
            item.encode(&mut vec);
        }
        vec
    }

    pub(crate) fn decode(vals : &[KVal]) -> FsResult<PhaseData>{
        if vals.len() == 0{
            Err("PhaseData decode failed")?;
        }
        let len = vals[0].ast_i64()? as usize;

        let mut items : Vec<PhaseDataItem> = vec_with_capacity_safe(len)?;
        let mut iter = vals[1..].into_iter();
        for _ in 0..len {
            let data = PhaseDataItem::decode(&mut iter)?;
            items.push(data);
        }
        Ok(PhaseData{ items })
    }

    pub(crate) fn get_nth_largest_files_size(&self, nth : usize) -> Option<u64>{
        let nth = if nth == 0{ 1 } else{ nth };

        let mut sizes : Vec<u64> = self.items.iter()
            .map(|item| item.my_size()).collect();

        sizes.sort_by_key(|a| *a);

        sizes.get(sizes.len() - nth).map(|a| *a)
    }
}

