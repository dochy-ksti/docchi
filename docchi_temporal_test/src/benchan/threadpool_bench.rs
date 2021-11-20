use std::sync::atomic::{AtomicI64, Ordering};
use test::Bencher;
use rand::Rng;
use std::sync::Arc;

static VEC_LEN : usize = 10;
static INNER_LEN : usize = 1_000_000;

fn vec64() -> Vec<Vec<u8>>{
    let mut bv = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..VEC_LEN {
        let mut inner = Vec::with_capacity(INNER_LEN);
        for _ in 0..INNER_LEN {
            inner.push(rng.gen());
        }
        bv.push(inner);
    }
    bv
}

fn arc_vec64() -> Vec<Arc<Vec<u8>>>{
    let mut bv = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..VEC_LEN {
        let mut inner = Vec::with_capacity(INNER_LEN);
        for _ in 0..INNER_LEN {
            inner.push(rng.gen());
        }
        bv.push(Arc::new(inner));
    }
    bv
}

#[bench]
fn bench_st(b : &mut Bencher){
    let bv = vec64();
    b.iter(|| {
        let mut sum = 0;
        for inner in &bv {
            for t in inner {
                if *t == 0 {
                    sum += 1;
                } else {
                    sum -= 1;
                }
            }
        }
        ATOM.fetch_add(sum, Ordering::Relaxed);
    });

    println!("{}", ATOM.load(Ordering::Relaxed));
}

#[bench]
fn bench_pool(b : &mut Bencher){
    let bv = arc_vec64();

    b.iter(|| {
        let (sender, receiver) = std::sync::mpsc::channel();

        for inner in &bv {
            let inner = inner.clone();
            let mut sender = sender.clone();
            rayon::spawn(move|| {
                let mut sum : i64 = 0;
                for t in inner.as_ref() {
                    if *t == 0 {
                        sum += 1;
                    } else {
                        sum -= 1;
                    }
                }
                match sender.send(sum){
                    Ok(_) =>{},
                    Err(e) =>{ println!("send error {}", e.to_string())}
                }
            });
        }
        let mut sum = 0;
        for _ in 0..VEC_LEN{
            sum += receiver.recv().unwrap();
        }
        ATOM.fetch_add(sum, Ordering::Relaxed);
    });

    println!("{}", ATOM.load(Ordering::Relaxed));;
}

static ATOM: AtomicI64 = AtomicI64::new(0);

//static VEC_LEN : usize = 100_000;
// static INNER_LEN : usize = 100;
//-3010000000
// test benchan::threadpool_bench::bench_pool ... bench:  35,291,230 ns/iter (+/- 3,092,987)
// -6020000000
// test benchan::threadpool_bench::bench_st   ... bench:   9,004,800 ns/iter (+/- 172,166)

// static VEC_LEN : usize = 10_000;
// static INNER_LEN : usize = 1000;
// -9010000000
// test benchan::threadpool_bench::bench_pool ... bench:   5,283,245 ns/iter (+/- 466,750)
// -18020000000
// test benchan::threadpool_bench::bench_st   ... bench:   6,371,420 ns/iter (+/- 173,987)

// static VEC_LEN : usize = 1_000;
// static INNER_LEN : usize = 10000;
// -9010000000
// test benchan::threadpool_bench::bench_pool ... bench:   4,292,455 ns/iter (+/- 226,315)
// -18020000000
// test benchan::threadpool_bench::bench_st   ... bench:   5,943,635 ns/iter (+/- 157,499)

// static VEC_LEN : usize = 100;
// static INNER_LEN : usize = 100_000;
// -9010000000
// test benchan::threadpool_bench::bench_pool ... bench:   4,129,780 ns/iter (+/- 168,445)
// -18020000000
// test benchan::threadpool_bench::bench_st   ... bench:   5,944,530 ns/iter (+/- 187,824)

// static VEC_LEN : usize = 10;
// static INNER_LEN : usize = 1_000_000;
// -9010000000
// test benchan::threadpool_bench::bench_pool ... bench:   4,058,960 ns/iter (+/- 287,905)
// -18020000000
// test benchan::threadpool_bench::bench_st   ... bench:   5,982,230 ns/iter (+/- 534,526)

// static VEC_LEN : usize = 4;
// static INNER_LEN : usize = 2_500_000;
// -9010000000
// test benchan::threadpool_bench::bench_pool ... bench:   4,058,940 ns/iter (+/- 478,486)
// -18020000000
// test benchan::threadpool_bench::bench_st   ... bench:   5,972,505 ns/iter (+/- 193,141)

// static VEC_LEN : usize = 1;
// static INNER_LEN : usize = 10_000_000;
// -9010000000
// test benchan::threadpool_bench::bench_pool ... bench:   5,824,540 ns/iter (+/- 175,283)
// -18020000000
// test benchan::threadpool_bench::bench_st   ... bench:   5,979,925 ns/iter (+/- 143,011)

// Vec<u8>に変更
// static VEC_LEN : usize = 100;
// static INNER_LEN : usize = 100_000;
// -20846021152
// test benchan::threadpool_bench::bench_pool ... bench:   1,825,120 ns/iter (+/- 192,431)
// -29785193542
// test benchan::threadpool_bench::bench_st   ... bench:   6,216,150 ns/iter (+/- 106,515)
// やはりメモリ転送がボトルネックになってマルチスレッド化の効果が出ていなかった

// static VEC_LEN : usize = 4;
// static INNER_LEN : usize = 2_500_000;
// -20844739542
// test benchan::threadpool_bench::bench_pool ... bench:   1,675,912 ns/iter (+/- 150,170)
// -29783816426
// test benchan::threadpool_bench::bench_st   ... bench:   6,200,145 ns/iter (+/- 117,933)

// static VEC_LEN : usize = 10;
// static INNER_LEN : usize = 1_000_000;
// -2986482268
// test benchan::threadpool_bench::bench_pool ... bench:   2,004,320 ns/iter (+/- 526,821)
// -5972969954
// test benchan::threadpool_bench::bench_st   ... bench:   6,204,840 ns/iter (+/- 160,441)
// 4コアだから、10スレッドだと割り切れなくて遅くなるね・・・