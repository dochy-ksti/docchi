use test::Bencher;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, Ordering, AtomicI64};
use bitvec::prelude::BitVec;
use rand::Rng;

static VEC_LEN : usize = 10_000_000;

fn bitvec() -> BitVec {
    let mut bv = BitVec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..VEC_LEN {
        bv.push(rng.gen());
    }
    bv
}

fn vec8() -> Vec<u8>{
    let mut bv = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..VEC_LEN {
        bv.push(rng.gen());
    }
    bv
}

fn vec64() -> Vec<u64>{
    let mut bv = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..VEC_LEN {
        bv.push(rng.gen());
    }
    bv
}

#[bench]
fn bench_bitvec(b : &mut Bencher){
    let bv = bitvec();
    b.iter(|| {
        let mut sum : i64 = 0;
        for t in &bv{
            if *t{
                sum += 1;
            } else{
                sum -= 1;
            }
        }
        ATOM.fetch_add(sum, Ordering::Relaxed);
        //println!("{}", sum);
    });

    println!("{}", ATOM.load(Ordering::Relaxed));;
}

#[bench]
fn bench_vec8(b : &mut Bencher){
    let bv = vec8();
    b.iter(|| {
        let mut sum : i64 = 0;
        for t in &bv{
            if *t == 0{
                sum += 1;
            } else{
                sum -= 1;
            }
        }
        ATOM.fetch_add(sum, Ordering::Relaxed);
        //println!("{}", sum);
    });

    println!("{}", ATOM.load(Ordering::Relaxed));;
}

#[bench]
fn bench_vec64(b : &mut Bencher){
    let bv = vec64();
    b.iter(|| {
        let mut sum : i64 = 0;
        for t in &bv{
            if *t == 0{
                sum += 1;
            } else{
                sum -= 1;
            }
        }
        ATOM.fetch_add(sum, Ordering::Relaxed);
        //println!("{}", sum);
    });

    println!("{}", ATOM.load(Ordering::Relaxed));
}

static ATOM: AtomicI64 = AtomicI64::new(0);

// 1307544
// test benchan::benchanmark::bench_bitvec ... bench:  19,070,080 ns/iter (+/- 554,635)
// -9008692456
// test benchan::benchanmark::bench_vec64  ... bench:   5,758,890 ns/iter (+/- 160,371)
// -11995089240
// test benchan::benchanmark::bench_vec8   ... bench:   6,189,500 ns/iter (+/- 144,095)