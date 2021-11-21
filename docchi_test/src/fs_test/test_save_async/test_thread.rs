use docchi::error::DpResult;
use docchi::fs::common::{CurrentSrc};
use std::path::{Path, PathBuf};
use docchi::core::structs::RootObject;
use rand::Rng;
use std::time::Duration;
//use std::lazy::Lazy;

use once_cell::sync::Lazy;
use std::io::Write;


//use parking_lot::FairMutex as Bmutex;
use parking_lot::Mutex as Bmutex;
//use std::sync::Mutex as Bmutex;

static vec_lazy : Lazy<Bmutex<Vec<String>>> = Lazy::new(||{
    Bmutex::new(Vec::new())
});

///thread::spawnでスレッドはバラバラに実行されることの確認
//#[test]
fn test_thread() -> DpResult<()> {

    let max = 10;


    for i in 0..max{
        std::thread::spawn(move||{
            {
                let mut v = vec_lazy.lock();


                std::thread::sleep(Duration::from_millis(100));

                v.push(format!("{}", i));
            }

            // let mut v = vec_lazy2.lock().unwrap();
            // v.push(format!("{}",i));
        });
    }

    std::thread::sleep(Duration::from_millis(1));

    let mut v = vec_lazy.lock();
    let hoge : &Vec<String> = &v;
    println!("{:?}", hoge);

    //Output:
    //["0", "6", "1", "2", "3", "5", "7", "4", "8", "9"]


    Ok(())
}