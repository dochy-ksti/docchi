use anyhow::Result;
use std::time::Duration;
//use std::lazy::Lazy;

use once_cell::sync::Lazy;


//use parking_lot::FairMutex as Bmutex;
use parking_lot::Mutex as Bmutex;
//use std::sync::Mutex as Bmutex;

static VEC_LAZY: Lazy<Bmutex<Vec<String>>> = Lazy::new(||{
    Bmutex::new(Vec::new())
});

///thread::spawnでスレッドはバラバラに実行されることの確認
//#[test]
fn test_thread() -> Result<()> {

    let max = 10;


    for i in 0..max{
        std::thread::spawn(move||{
            {
                let mut v = VEC_LAZY.lock();


                std::thread::sleep(Duration::from_millis(100));

                v.push(format!("{}", i));
            }

            // let mut v = vec_lazy2.lock().unwrap();
            // v.push(format!("{}",i));
        });
    }

    std::thread::sleep(Duration::from_millis(1));

    let v = VEC_LAZY.lock();
    let hoge : &Vec<String> = &v;
    println!("{:?}", hoge);

    //Output:
    //["0", "6", "1", "2", "3", "5", "7", "4", "8", "9"]


    Ok(())
}