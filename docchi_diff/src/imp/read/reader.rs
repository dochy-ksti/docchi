use docchi_compaction::kval_enum::KVal;
use crate::diff_error::{DiffError, OptToErr};
use with_capacity_safe::vec_with_capacity_safe;

pub(crate) struct Reader{
    vec : std::vec::IntoIter<KVal>,
}

impl Reader{
    pub(crate) fn new(vec : Vec<KVal>) -> Reader{
        Reader{ vec : vec.into_iter()}
    }

    pub(crate) fn read(&mut self) -> Result<KVal, DiffError>{
        //let index = self.ind;

        //self.ind += 1;
        Ok(self.vec.next().ok_or("reached the end of the data")?)
    }

    pub(crate) fn read_u64_array(&mut self) -> Result<Vec<u64>, DiffError>{
        let len = self.read()?.ast_i64()? as usize;
        let mut vec = vec_with_capacity_safe(len)?;
        for _ in 0..len{
            vec.push(self.read()?.ast_i64()? as u64);
        }
        Ok(vec)
    }

    pub(crate) fn read_usize_array(&mut self) -> Result<Vec<usize>, DiffError>{
        let len = self.read()?.ast_i64()? as usize;
        let mut vec = vec_with_capacity_safe(len)?;
        for _ in 0..len{
            vec.push(self.read()?.ast_i64()? as usize);
        }
        Ok(vec)
    }
}