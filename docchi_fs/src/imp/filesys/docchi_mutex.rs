use crate::imp::filesys::save_cache_item::SaveCacheItem;
use std::sync::atomic::Ordering;
use std::sync::MutexGuard;

pub(crate) struct DocchiMutex<'a>{
    _guard: MutexGuard<'a, ()>,
    cache: &'a SaveCacheItem,
}
impl<'a> DocchiMutex<'a>{
    pub(crate) fn new<'b>(guard : MutexGuard<'b, ()>,
                          cache : &'b SaveCacheItem) -> DocchiMutex<'b>{
        cache.queued_atomic().fetch_add(1, Ordering::Relaxed);
        DocchiMutex { _guard: guard, cache }
    }
    pub(crate) fn cache(&self) -> &SaveCacheItem{ self.cache }
}
impl<'a> Drop for DocchiMutex<'a>{
    fn drop(&mut self) {
        self.cache().queued_atomic().fetch_sub(1, Ordering::Relaxed);
    }
}