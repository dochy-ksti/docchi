use std::sync::atomic::{AtomicI64, Ordering};
use test::Bencher;
use rand::Rng;
use std::sync::Arc;

static VEC_LEN : usize = 100000;
static INNER_LEN : usize = 2;

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

//1 task あたり約340nsかかる 1/3 micro second
//非力な環境を考えて1 micro secondと換算してもよいだろう

//static VEC_LEN : usize = 100_000;
//static INNER_LEN : usize = 100;
//-2986452770
// test benchan::threadpool_bench::bench_pool ... bench:  34,644,190 ns/iter (+/- 1,735,788)
// -11925531456
// test benchan::threadpool_bench::bench_st   ... bench:   8,289,375 ns/iter (+/- 461,029)

//static VEC_LEN : usize = 10_000;
// static INNER_LEN : usize = 1_000;
//-8939476928
// test benchan::threadpool_bench::bench_pool ... bench:   4,498,805 ns/iter (+/- 518,372)
// -17879566536
// test benchan::threadpool_bench::bench_st   ... bench:   7,072,615 ns/iter (+/- 201,394)

//static VEC_LEN : usize = 1_000;
// static INNER_LEN : usize = 10_000;
//-20844945440
// test benchan::threadpool_bench::bench_pool ... bench:   2,458,767 ns/iter (+/- 263,929)
// -29785004414
// test benchan::threadpool_bench::bench_st   ... bench:   6,758,105 ns/iter (+/- 172,024)

//static VEC_LEN : usize = 100;
// static INNER_LEN : usize = 100_000;
//-20845554730
// test benchan::threadpool_bench::bench_pool ... bench:   2,062,895 ns/iter (+/- 272,761)
// -29784975796
// test benchan::threadpool_bench::bench_st   ... bench:   6,748,885 ns/iter (+/- 139,985)

//static VEC_LEN : usize = 10;
// static INNER_LEN : usize = 1_000_000;
//-20845235378
// test benchan::threadpool_bench::bench_pool ... bench:   2,074,070 ns/iter (+/- 115,883)
// -29785198846
// test benchan::threadpool_bench::bench_st   ... bench:   6,755,415 ns/iter (+/- 187,608)

//static VEC_LEN : usize = 4;
//static INNER_LEN : usize = 2_500_000;
//-20846390928
// test benchan::threadpool_bench::bench_pool ... bench:   2,066,365 ns/iter (+/- 245,362)
// -29785784964
// test benchan::threadpool_bench::bench_st   ... bench:   6,732,140 ns/iter (+/- 188,267)

//static VEC_LEN : usize = 2;
// static INNER_LEN : usize = 5_000_000;
//-8939822912
// test benchan::threadpool_bench::bench_pool ... bench:   3,406,975 ns/iter (+/- 148,196)
// -17879236770
// test benchan::threadpool_bench::bench_st   ... bench:   6,745,165 ns/iter (+/- 513,804)