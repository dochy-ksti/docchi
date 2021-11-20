use crate::FsResult;
use crate::imp::history::current_root_obj_info::fifo_thread::FifoThread;
use std::thread::sleep;
use std::time::Duration;

//#[test]
fn test_fifo_thread() -> FsResult<()>{
    let th = FifoThread::new();
    for i in 0..100 {
        th.spawn_fifo(move ||{
            println!("{}",i);
            sleep(Duration::from_millis(100));
        });
    }
    sleep(Duration::from_millis(10000));
    Ok(())
}

