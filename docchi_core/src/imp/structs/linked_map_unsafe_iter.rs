use crate::imp::structs::linked_m::{LinkedMap, MutNode, get_next, get_prev, ptr_eq};
use std::ptr::null_mut;


/// LinkedMapIterMutを作るために、unsafeイテレータとPhantomDataを使ってインチキしないとならなかった
/// これをナマで使うのは危険だが、Safe版であるLinkedMapIterMutでライフタイム付けて使うぶんには問題ない
#[derive(Debug)]
pub(crate) struct LinkedMapUnsafeIter<V>{
    map : *const LinkedMap<V>,
    node : *const MutNode<V>,
}
impl<V> LinkedMapUnsafeIter<V>{
    ///IterMutで使う場合は、&mut から *mut ポインタを取得しておく必要がある
    pub(crate) fn new(map : *const LinkedMap<V>, node : *const MutNode<V>) -> LinkedMapUnsafeIter<V>{ LinkedMapUnsafeIter{ map, node } }

    ///現在のカーソルにあるアイテムを返し、カーソルを進める
    pub fn next_const<'a>(&mut self) -> Option<(&'a u64, &'a V)> {
        self.next_impl().map(|current_node| {
            //next_mutからキャストするほうが楽なんだけど、UnsafeIterが&LinkedMapから作られる場合があり、その場合&mutにした時点で（書き換えなくても)UBになる
            //https://github.com/rust-lang/rust-clippy/issues/4774#issuecomment-565651216
            //これで回避できているはず・・・
            let node = unsafe { &* current_node };
            (&node.id, &node.item)
        })
    }

    ///現在のカーソルにあるアイテムを返し、カーソルを進める。mapが&LinkedMapだった場合、&'a mut Vを書き換えるとUndefined Behavior
    pub fn next_mut<'a>(&mut self) -> Option<(&'a u64, &'a mut V)> {
        self.next_impl().map(|current_node| {
            let node = unsafe { &mut *current_node };
            (&node.id, &mut node.item)
        })
    }

    fn next_impl(&mut self) -> Option<*mut MutNode<V>>{
        if self.node.is_null() { return None; }
        let current_node = self.node as *mut MutNode<V>;
        let map = unsafe{ &*self.map};
        if ptr_eq(self.node, map.last) {
            self.node = null_mut();
        } else {
            self.node = get_next(self.node);
        }
        Some(current_node)
    }

    ///前に戻ることが出来る。そして元あった場所を削除し、それによって削除されたアイテムの次にあったアイテムが現在のカーソルの次にくるので、
    /// next2回でそれをとることも出来る。
    ///今ある場所をremoveしたらポインタが不正になって安全にnext/prevできない
    pub fn prev_const<'a>(&mut self) -> Option<(&'a u64, &'a V)> {
        self.prev_impl().map(|current_node|{
            let node = unsafe{ &*current_node };
            (&node.id, &node.item)
        })

    }

    ///前に戻ることが出来る。そして元あった場所を削除し、それによって削除されたアイテムの次にあったアイテムが現在のカーソルの次にくるので、
    /// next2回でそれをとることも出来る。
    ///今ある場所をremoveしたらポインタが不正になって安全にnext/prevできない
    pub fn prev_mut<'a>(&mut self) -> Option<(&'a u64, &'a mut V)> {
        self.prev_impl().map(|current_node|{
            let node = unsafe{ &mut *current_node };
            (&node.id, &mut node.item)
        })
    }

    fn prev_impl(&mut self) -> Option<*mut MutNode<V>> {
        if self.node.is_null(){ return None; }
        let current_node = self.node as *mut MutNode<V>;
        let map = unsafe{ &*self.map };
        if ptr_eq(self.node, map.first){
            self.node = null_mut();
        } else {
            self.node = get_prev(self.node);
        }
        Some(current_node)
    }

    pub fn current_mut<'a>(&mut self) -> Option<(&'a u64, &'a mut V)> {
        if self.node.is_null(){ return None; }
        let node = unsafe{ &mut *(self.node as *mut MutNode<V>) };
        return Some((&node.id, &mut node.item))
    }

    pub fn current_const<'a>(&mut self) -> Option<(&'a u64, &'a V)> {
        if self.node.is_null(){ return None; }
        let node = unsafe{ & *self.node };
        return Some((&node.id, &node.item))
    }

    ///nextもprevも現在のカーソルにあるアイテムを返す
    ///空であるか、最後/最初まで移動してアイテムが無くなったらfalse
    pub fn is_available(&self) -> bool {
        !self.node.is_null()
    }

    pub fn is_first(&self) -> bool {
        if self.node.is_null(){ return false; }
        let map = unsafe{ &*self.map };
        let node = unsafe{ &*self.node };
        if let Some(id) = map.first_id(){
            return id == node.id;
        } else{
            return false;
        }
    }

    pub fn is_last(&self) -> bool {
        if self.node.is_null(){ return false; }
        let map = unsafe{ &*self.map };
        let node = unsafe{ &*self.node };
        if let Some(id) = map.last_id(){
            return id == node.id;
        } else{
            return false;
        }
    }
}