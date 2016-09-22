use std::thread;
use std::vec;
use std::i64;
use rand::SeedableRng;
use rand::Rng;
extern crate rand;
extern crate time;
use rand::distributions::{Range,IndependentSample};
extern crate num_cpus;

fn minMaxTuple(t1: (i64, i64), t2: (i64, i64)) -> (i64, i64) {
    return (if t1.0 < t2.0 {t1.0} else {t2.0}, if t1.1 > t2.1 {t1.1} else {t2.1});
}

fn getMinMax(n: u64) -> (i64, i64) {
    let millis = time::precise_time_ns() as usize; 
    let mut rng = rand::StdRng::from_seed(&[millis]);
    let values = (0..n).map(|_| rng.gen::<i64>()).collect::<Vec<_>>();

    let min = i64::MAX;
    let max = 0;
    for value in values {
        min = if value < min {value}  else {min};
        max = if value > max {value}  else {max};
    }
    (min, max)
}

fn getMinMaxPar(n:u64, nthreads:u64) -> (i64, i64) {
    let iters = n/nthreads;
    let last_one: bool = n % nthreads != 0;
    if last_one {
        nthreads -= 1;
    }

    let mut threads = (0..nthreads).map(move |_| thread::spawn(|| getMinMax(iters))).collect();
    if last_one {
        let iters = n - (iters * nthreads);
        let t_handle = thread::spawn(|| getMinMax(iters));
        threads.push(t_handle);
    }
    return threads.iter().map(|h| h.join())
                         .fold((i64::MAX,0), minMaxTuple);
}

fn main() {
    println!("Hello, world!");
}
