use threadpool::ThreadPool;
use std::sync::Mutex;

pub(crate) struct FifoThread{
    pool : Mutex<Option<ThreadPool>>,
}

impl FifoThread{
    pub fn new() -> FifoThread{
        FifoThread{ pool : Mutex::new(None) }
    }

    pub fn spawn_fifo<F : FnOnce() + Send + 'static>(&self, f : F){
        let mut opt = self.pool.lock().unwrap();
        if opt.is_none(){
            *opt = Some(ThreadPool::new(1))
        }
        if let Some(p) = opt.as_ref(){
            p.execute(f);
        }
    }
}