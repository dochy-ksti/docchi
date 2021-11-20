//use crate::{HashM, HashMt};
use std::ptr::{null_mut};
use std::collections::VecDeque;
use crate::imp::structs::util::hash_m::{HashM, HashMt};
use crate::imp::structs::linked_map_unsafe_iter::LinkedMapUnsafeIter;
use crate::imp::structs::linked_map_iter::LinkedMapIter;
use crate::imp::structs::linked_map_iter_mut::LinkedMapIterMut;


unsafe impl<V> Send for LinkedMap<V> {}
unsafe impl<V> Sync for LinkedMap<V> {}

#[derive(Debug)]
pub struct LinkedMap<V>{
    pub(crate) map : HashM<u64, Box<MutNode<V>>>,
    pub(crate) first : *mut MutNode<V>,
    pub(crate) last : *mut MutNode<V>,
    pub(crate) next_id : u64,
}


#[derive(Debug)]
pub(crate) struct MutNode<V>{
    pub(crate) prev : *mut MutNode<V>,
    pub(crate) next : *mut MutNode<V>,
    pub(crate) item : V,
    pub(crate) id : u64,
}
impl<V : Clone> Clone for MutNode<V>{
    fn clone(&self) -> Self {
        MutNode{ prev : null_mut(), next : null_mut(), item : self.item.clone(), id : self.id }
    }
}
impl<V : PartialEq> PartialEq for MutNode<V>{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.item.eq(&other.item)
    }
}

impl<V> MutNode<V>{
    fn new(id : u64, item : V) -> MutNode<V>{
        MutNode{ item, prev : null_mut(), next : null_mut(), id }
    }
}
pub(crate) fn get_next<V>(this : *const MutNode<V>) ->*mut MutNode<V>{
    let node = unsafe{ &*this };
    node.next
}
fn set_next<V>(this : *mut MutNode<V>, next : *mut MutNode<V>){
    let node = unsafe{ this.as_mut().unwrap() };
    node.next = next;
}
pub(crate) fn get_prev<V>(this : *const MutNode<V>) -> *mut MutNode<V>{
    let node = unsafe{ &*this };
    node.prev
}
fn set_prev<V>(this : *mut MutNode<V>, prev : *mut MutNode<V>){
    let node = unsafe{ this.as_mut().unwrap() };
    node.prev = prev;
}
fn get_id<V>(this : *const MutNode<V>) -> u64{
    let node = unsafe{ &*this };
    node.id
}
fn get_item<'a, V>(this : *const MutNode<V>) -> &'a V{
    let node = unsafe{ &*this };
    &node.item
}
fn get_item_mut<'a, V>(this : *mut MutNode<V>) ->&'a mut V{
    let node = unsafe{ &mut *this };
    &mut node.item
}


pub(crate) fn ptr_eq<T>(l : *const T, r : *const T) -> bool{ std::ptr::eq(l,r) }

impl<V> LinkedMap<V> {

    pub fn new() -> LinkedMap<V> {
        LinkedMap { map : HashMt::new(), first : null_mut(), last : null_mut(), next_id : 0, }
    }
    pub fn construct(items : Vec<(u64,V)>, next_id : u64) -> LinkedMap<V>{
        if items.is_empty(){
            return LinkedMap::new();
        }
        let mut map : HashM<u64,Box<MutNode<V>>> = HashMt::with_capacity(items.len());
        let mut iter = items.into_iter();
        let (id,val) = iter.next().unwrap();
        let mut first_node  = Box::new(MutNode::new(id, val));
        let first_node_ptr = first_node.as_mut() as *mut _;
        let mut prev_node_ptr = first_node_ptr;
        map.insert(id, first_node);
        for (id,val) in iter{
            let mut node = Box::new(MutNode::new( id, val));
            set_next(prev_node_ptr, node.as_mut());
            node.as_mut().prev = prev_node_ptr;
            prev_node_ptr = node.as_mut();
            map.insert(id, node);
        }
        LinkedMap{ map, first : first_node_ptr, last : prev_node_ptr, next_id }
    }
    fn deconstruct(self) -> HashM<u64, Box<MutNode<V>>>{
        self.map
    }
    pub fn next_id(&self) -> u64{ self.next_id }
    pub fn set_next_id(&mut self, next_id : u64){ self.next_id = next_id }

    pub fn first(&self) -> Option<&V> {
        if self.first.is_null() { None } else { Some(get_item(self.first)) }
    }
    pub fn first_mut(&mut self) -> Option<&mut V> {
        if self.first.is_null() { None } else { Some(get_item_mut(self.first)) }
    }
    pub fn first_id(&self) -> Option<u64> {
        if self.first.is_null() { None } else { Some(get_id(self.first)) }
    }
    pub fn last(&self) -> Option<&V> {
        if self.last.is_null() { None } else { Some(get_item(self.last)) }
    }
    pub fn last_mut(&mut self) -> Option<&mut V> {
        if self.last.is_null() { None } else { Some(get_item_mut(self.last)) }
    }
    pub fn last_id(&self) -> Option<u64> {
        if self.last.is_null() { None } else { Some(get_id(self.last)) }
    }

    fn node_from_id_mut(&mut self, id : u64) -> Option<&mut MutNode<V>>{ self.map.get_mut(&id).map(|b| b.as_mut()) }
    fn node_from_id(&self, id : u64) -> Option<&MutNode<V>>{ self.map.get(&id).map(|b| b.as_ref()) }

    pub fn get_item(&self, id : u64) -> Option<&V>{ self.map.get(&id).map(|b| &b.as_ref().item) }
    pub fn get_item_mut(&mut self, id : u64) -> Option<&mut V>{ self.map.get_mut(&id).map(|b| &mut b.as_mut().item) }

    pub fn contains_key(&self, key : u64) -> bool{ self.map.contains_key(&key) }
    pub fn len(&self) -> usize{ self.map.len() }
    pub fn is_empty(&self) -> bool{ self.map.is_empty() }

    fn pull_last(&mut self) -> *mut MutNode<V>{
        if self.last.is_null(){ return null_mut() }
        let last = unsafe{ self.last.as_mut().unwrap() };
        if last.prev.is_null(){
            self.last = null_mut();
            self.first = null_mut();
            return last;
        } else{
            self.last = last.prev;
            set_next(self.last, null_mut());
            last.prev = null_mut();
            return last;
        }
    }

    fn pull_first(&mut self) -> *mut MutNode<V>{
        if self.first.is_null(){ return null_mut() }
        let first = unsafe{ self.first.as_mut().unwrap() };
        if first.next.is_null(){
            self.first = null_mut();
            self.last = null_mut();
            return first;
        } else{
            self.first = first.next;
            set_prev(self.first, null_mut());
            first.next = null_mut();
            return first;
        }
    }

    fn pull(&mut self, id : u64) -> *mut MutNode<V>{
        let node = if let Some(node) = self.node_from_id_mut(id){ node as *mut _ } else{ return null_mut(); };
        if ptr_eq(node, self.first){ return self.pull_first(); }
        if ptr_eq(node, self.last){ return self.pull_last(); }
        //firstでもlastでもないということは、中間ということ
        let prev = get_prev(node);
        let next = get_next(node);
        set_next(prev, next);
        set_prev(next, prev);
        set_next(node, null_mut());
        set_prev(node, null_mut());
        return node;
    }

    fn put_first(&mut self, node : *mut MutNode<V>){
        if self.first.is_null(){
            self.first = node;
            self.last = node;
        } else{
            if ptr_eq(self.first, node){ panic!() }

            let next = self.first;
            self.first = node;
            set_next(self.first, next);
            set_prev(next, self.first);
        }
    }

    fn put_last(&mut self, node : *mut MutNode<V>){
        if self.last.is_null(){
            self.first = node;
            self.last = node;
        } else{
            if ptr_eq(self.last, node){ panic!() }

            let prev = self.last;
            self.last = node;
            set_next(prev, self.last);
            set_prev(self.last, prev);
        }
    }

    fn put_next(&mut self, prev : *mut MutNode<V>, node : *mut MutNode<V>){
        if ptr_eq(prev, node){ panic!() }
        if self.last.is_null(){ panic!() }

        if ptr_eq(prev, self.last){
            return self.put_last(node);
        }
        let next = get_next(prev);
        set_next(prev, node);
        set_prev(node, prev);
        set_next(node, next);
        set_prev(next, node);
    }

    fn put_prev(&mut self, next : *mut MutNode<V>, node : *mut MutNode<V>){
        if ptr_eq(next, node){ panic!() }
        if self.first.is_null(){ panic!() }

        if ptr_eq(next, self.first){
            return self.put_first(node);
        }

        let prev = get_prev(next);
        set_next(prev, node);
        set_prev(node, prev);
        set_next(node, next);
        set_prev(next, node);
    }

    pub fn insert(&mut self, val : V) -> u64{
        self.insert_last(val)
    }

    pub fn insert_last(&mut self, val : V) -> u64{
        let mut node = Box::new(MutNode::new(self.next_id, val));
        self.put_last(node.as_mut());
        self.map.insert(self.next_id, node);
        self.next_id += 1;
        return self.next_id - 1;
    }

    /// idを指定してfirstにinsert.
    /// next_idとの整合性は簡単に壊れうるので注意
    /// idがかぶった場合、挿入は行われず、falseが返る
    pub fn insert_first_with_id(&mut self, id : u64, val : V) -> bool {
        //entryもやってみたがうまくいかない。safe rustではこれが限界か
        if self.map.contains_key(&id) { return false; }

        let mut node = Box::new(MutNode::new(id, val));
        self.put_first(node.as_mut());
        self.map.insert(id,node);
        true
    }

    /// idを指定してfirstにinsert.
    /// next_idとの整合性は簡単に壊れうるので注意
    /// idがかぶった場合、挿入は行われず、falseが返る
    pub fn insert_first_with_id_unchecked(&mut self, id : u64, val : V){
        let mut node = Box::new(MutNode::new(id, val));
        self.put_first(node.as_mut());
        if self.map.insert(id,node).is_some(){
            panic!("linked_m: insert_first_with_id_unchecked: list has an item with the same ID");
        }
    }

    /// idを指定してlastにinsert.
    /// next_idとの整合性は簡単に壊れうるので注意
    /// idがかぶった場合、挿入は行われず、falseが返る
    pub fn insert_last_with_id(&mut self, id : u64, val : V) -> bool {
        //entryもやってみたがうまくいかない。safe rustではこれが限界か
        if self.map.contains_key(&id) { return false; }

        let mut node = Box::new(MutNode::new(id, val));
        self.put_last(node.as_mut());
        self.map.insert(id,node);
        true
    }

    /// idを指定してlastにinsert.
    /// next_idとの整合性は簡単に壊れうるので注意
    /// idがかぶった場合パニックする
    pub fn insert_last_with_id_unchecked(&mut self, id : u64, val : V){
        let mut node = Box::new(MutNode::new(id, val));
        self.put_last(node.as_mut());
        if self.map.insert(id,node).is_some(){
            panic!("linked_m: insert_last_with_id_unchecked: list has an item with the same ID");
        }
    }


    pub fn insert_first(&mut self, val : V) -> u64{
        let mut node = Box::new(MutNode::new(self.next_id, val));
        self.put_first(node.as_mut());
        self.map.insert(self.next_id, node);
        self.next_id += 1;
        return self.next_id - 1;
    }

    pub fn remove(&mut self, id : u64) -> bool {
        let node = self.pull(id);
        if node.is_null(){ return false; }
        self.map.remove(&id);
        return true;
    }
    pub fn remove_first(&mut self) -> bool{
        let node = self.pull_first();
        if node.is_null(){ return false; }
        self.map.remove(&get_id(node));
        return true;
    }

    pub fn remove_last(&mut self) -> bool{
        let node = self.pull_last();
        if node.is_null(){ return false; }
        self.map.remove(&get_id(node));
        return true;
    }

    pub fn move_to_first(&mut self, id : u64) -> bool {
        let node = self.pull(id);
        if node.is_null(){ return false; }
        self.put_first(node);
        return true;
    }

    pub fn move_to_last(&mut self, id : u64) -> bool {
        let node = self.pull(id);
        if node.is_null(){ return false; }
        self.put_last(node);
        return true;
    }

    pub fn move_to_prev(&mut self, next_items_id : u64, id : u64) -> bool{
        if id == next_items_id{ return false; }
        let next = if let Some(node) = self.map.get_mut(&next_items_id).map(|b| b.as_mut() as *mut MutNode<V>){ node } else{ return false };
        let node = self.pull(id);
        if node.is_null(){ return false; }
        self.put_prev(next, node);
        return true;
    }

    pub fn move_to_next(&mut self, prev_items_id : u64, id : u64) -> bool{
        if id == prev_items_id{ return false; }
        let prev = if let Some(node) = self.map.get_mut(&prev_items_id).map(|b| b.as_mut() as *mut MutNode<V>){ node } else{ return false };
        let node = self.pull(id);
        if node.is_null(){ return false; }
        self.put_next(prev, node);
        return true;
    }

    pub fn iter(&self) -> LinkedMapIter<V> { LinkedMapIter::new(self, self.first) }
    pub fn iter_from_last(&self) -> LinkedMapIter<V>{ LinkedMapIter::new(self, self.last)}
    pub fn iter_from_id(&self, id : u64) -> Option<LinkedMapIter<V>>{
        self.node_from_id(id).map(|node| LinkedMapIter::new(self, node))
    }
    pub fn iter_mut(&mut self) -> LinkedMapIterMut<V> { LinkedMapIterMut::new(self, self.first) }
    pub fn iter_mut_from_last(&mut self) -> LinkedMapIterMut<V>{ LinkedMapIterMut::new(self, self.last)}
    pub fn iter_mut_from_id(&mut self, id : u64) -> Option<LinkedMapIterMut<V>>{
        let node = if let Some(node) = self.node_from_id_mut(id){ node as *mut MutNode<V> } else{ return None; };
        Some(LinkedMapIterMut::new(self, node))
    }

    pub(crate) unsafe fn iter_unsafe_mut(&mut self) -> LinkedMapUnsafeIter<V>{ LinkedMapUnsafeIter::new(self, self.first) }
    pub(crate) unsafe fn iter_from_last_unsafe_mut(&mut self) -> LinkedMapUnsafeIter<V>{ LinkedMapUnsafeIter::new(self, self.last) }
    pub(crate) unsafe fn iter_from_id_unsafe_mut(&mut self, id : u64) -> Option<LinkedMapUnsafeIter<V>>{
        let node = if let Some(node) = self.node_from_id_mut(id){ node as *mut MutNode<V> } else{ return None; };
        Some(LinkedMapUnsafeIter::new(self, node))
    }

    pub(crate) unsafe fn iter_unsafe_const(&self) -> LinkedMapUnsafeIter<V>{ LinkedMapUnsafeIter::new(self, self.first) }
    pub(crate) unsafe fn iter_from_last_unsafe_const(&self) -> LinkedMapUnsafeIter<V>{ LinkedMapUnsafeIter::new(self, self.last) }
    pub(crate) unsafe fn iter_from_id_unsafe_const(&self, id : u64) -> Option<LinkedMapUnsafeIter<V>>{
        let node = if let Some(node) = self.node_from_id(id){ node as *const MutNode<V> } else{ return None; };
        Some(LinkedMapUnsafeIter::new(self, node))

    }

    pub fn into_iter(self) -> LinkedMapIntoIter<V>{ LinkedMapIntoIter::new(self) }
}

impl<V : Clone> Clone for LinkedMap<V>{
    fn clone(&self) -> Self {
        if self.map.is_empty(){ return LinkedMap::new() }
        let mut map = self.map.clone();
        let first_node = map.get_mut(&self.first_id().unwrap()).unwrap().as_mut() as *mut _;
        let last_node = map.get_mut(&self.last_id().unwrap()).unwrap().as_mut() as *mut _;
        let mut iter = self.iter();
        iter.next();
        let mut prev_node = first_node;
        for (id, _) in iter{
            let current = map.get_mut(id).unwrap();
            set_next(prev_node, current.as_mut());
            current.as_mut().prev = prev_node;
            prev_node = current.as_mut();
        }
        LinkedMap{ map, first : first_node, last : last_node, next_id : self.next_id }
    }
}

impl<V : PartialEq> PartialEq for LinkedMap<V>{
    fn eq(&self, other: &Self) -> bool {
        if self.map.len() != other.map.len(){ return false; }
        let l = self.iter();
        let r = other.iter();
        for ((a,v1),(b,v2)) in l.zip(r){
             if *a != *b{ return false; }
            if v1 != v2{ return false; }
        }
         return true;
    }
}


// impl<'a,V> IntoIterator for &'a LinkedMap<V>{
//     type Item = (&'a u64, &'a V);
//     type IntoIter = LinkedMapIter<'a, V>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.iter()
//     }
// }
//
//

pub struct LinkedMapIntoIter<V>{
    map : HashM<u64, Box<MutNode<V>>>,
    deq : VecDeque<u64>,
}
impl<V> LinkedMapIntoIter<V>{
    pub fn new(map : LinkedMap<V>) -> LinkedMapIntoIter<V>{
        let deq : VecDeque<u64> = map.iter().map(|(k,_)| *k).collect();
        LinkedMapIntoIter{ map : map.deconstruct(), deq }
    }
}
impl<V> Iterator for LinkedMapIntoIter<V>{
    type Item = (u64,V);

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.deq.pop_front()?;
        let b = self.map.remove(&id)?;
        let node = *b;
        return Some((id,node.item));
    }
}

impl<V> IntoIterator for LinkedMap<V>{
    type Item = (u64, V);
    type IntoIter = LinkedMapIntoIter<V>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedMapIntoIter::new(self)
    }
}
