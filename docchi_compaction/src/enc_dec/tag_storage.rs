use bit_vec::BitVec;

pub(crate) struct TagStorage{
    pub vec : BitVec,

}

impl TagStorage{
    pub fn new() -> TagStorage{
        TagStorage{ vec : BitVec::new() }
    }
    ///n bitを追加。
    pub fn append(&mut self, val : u64, n : usize){
        if n == 0{ return; }
        if 64 < n{ panic!("you can only append 64 bits at a time"); }

        //1bitずつ追加していく
        let mut filter : u64 = 2u64.pow(n as u32 - 1);
        loop{
            let b = val & filter != 0;
            self.vec.push(b);
            filter = filter / 2;
            if filter == 0{ return; }
        }

    }

    pub fn to_vec(&self) -> Vec<u8>{
        self.vec.to_bytes()
    }
}

