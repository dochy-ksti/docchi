use crate::history::PeekableCacheInfo;
use std::ops::{Deref, DerefMut};
use crate::imp::history::current_root_obj_info::history_cache_item::SyncedItem;
use std::sync::atomic::Ordering;
use std::sync::MutexGuard;

pub(crate) struct MutexG<'a>{
    guard : MutexGuard<'a, SyncedItem>,
    peekable: &'a PeekableCacheInfo,
}
impl<'a> MutexG<'a>{
    pub(crate) fn new<'b>(guard : MutexGuard<'b, SyncedItem>,
                      peekable : &'b PeekableCacheInfo) -> MutexG<'b>{
        peekable.queued_atomic().fetch_add(1, Ordering::AcqRel);
        MutexG{ guard, peekable }
    }
    pub(crate) fn peekable(&self) -> &PeekableCacheInfo{ &self.peekable }
}
impl<'a> Drop for MutexG<'a>{
    fn drop(&mut self) {
        self.peekable().queued_atomic().fetch_sub(1, Ordering::AcqRel);
    }
}

impl<'a> Deref for MutexG<'a>{
    type Target = SyncedItem;

    fn deref(&self) -> &Self::Target {
        Deref::deref(&self.guard)
    }
}

impl<'a> DerefMut for MutexG<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        DerefMut::deref_mut(&mut self.guard)
    }
}