use crate::imp::structs::linked_m::{LinkedMap, MutNode};
use crate::imp::structs::linked_map_unsafe_iter::LinkedMapUnsafeIter;
use std::marker::PhantomData;


pub struct LinkedMapIter<'a, V>{
    iter : LinkedMapUnsafeIter<V>,
    phantom : PhantomData<&'a LinkedMap<V>>,
}
impl<'a, V> LinkedMapIter<'a, V>{
    pub(crate) fn new(map : &'a LinkedMap<V>, node : *const MutNode<V>) -> LinkedMapIter<'a, V>{
        //LinkedMapIterが有効な間に書き換えるとアウトだが、&が有効なはずなので大丈夫
        LinkedMapIter{ iter : LinkedMapUnsafeIter::new(map, node), phantom : PhantomData::default() }
    }

    ///現在のカーソルにあるアイテムを返し、カーソルを進める
    pub fn next(&mut self) -> Option<(&'a u64, &'a V)> {
        self.iter.next_const()
    }

    ///前に戻ることが出来る。そして元あった場所を削除し、それによって削除されたアイテムの次にあったアイテムが現在のカーソルの次にくるので、
    /// next2回でそれをとることも出来る。Cインターフェースやunsafe iterなら
    ///今ある場所をremoveしたらポインタが不正になって安全にnext/prevできない
    pub fn prev(&mut self) -> Option<(&'a u64, &'a V)> {
        self.iter.prev_const()
    }

    pub fn current(&mut self) -> Option<(&'a u64, &'a V)> {
        self.iter.current_const()
    }

    ///nextもprevも現在のカーソルにあるアイテムを返す
    ///空であるか、最後/最初まで移動してアイテムが無くなったらfalse
    pub fn is_available(&self) -> bool {
        self.iter.is_available()
    }

    pub fn is_first(&self) -> bool {
        self.iter.is_first()
    }
    pub fn is_last(&self) -> bool {
        self.iter.is_first()
    }
}
impl<'a,V> Iterator for LinkedMapIter<'a, V>{
    type Item = (&'a u64, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
impl<'a,V> IntoIterator for &'a LinkedMap<V>{
    type Item = (&'a u64, &'a V);
    type IntoIter = LinkedMapIter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}