use std::thread;
use std::u64;
use rand::SeedableRng;

use std::io::prelude::*;
use std::fs::File;

use rand::Rng;
extern crate rand;
extern crate time;
extern crate num_cpus;

fn min_max_tuple(t1: (u64, u64), t2: (u64, u64)) -> (u64, u64) {
    return (if t1.0 < t2.0 {t1.0} else {t2.0}, if t1.1 > t2.1 {t1.1} else {t2.1});
}

fn get_min_max(n: u64) -> (u64, u64) {
    if n == 0 {
        return (0,0);
    }
    let millis = time::precise_time_ns() as usize; 
    let mut rng = rand::StdRng::from_seed(&[millis]);
    let values = (0..n).map(|_| rng.gen::<u64>()).collect::<Vec<_>>();

    let mut min = values.get(0).unwrap().clone();
    let mut max = values.get(0).unwrap().clone();
    for value in values {
        min = if value < min {value}  else {min};
        max = if value > max {value}  else {max};
    }
    (min, max)
}

fn get_min_max_par(n:u64, nt:u64) -> (u64, u64) {
    let mut nthreads = nt;
    let iters = n/nthreads;
    
    let last_one: bool = n % nthreads != 0;
    if last_one {
        nthreads -= 1;
    }

    let mut threads = (0..nthreads).map(move |_| thread::spawn(move || get_min_max(n/nthreads))).collect::<Vec<_>>();
    if last_one {
        let iters = n - (iters * nthreads);
        let t_handle = thread::spawn(move || get_min_max(iters));
        threads.push(t_handle);
    }
   threads.into_iter()
          .map(move |h| h.join().unwrap())
          .fold((u64::MAX,0), min_max_tuple)
}
fn generate_perf_data(iters: u64, times_to_run: u64, max_threads: u64, filename: String) {
    let results = (1..max_threads+1).map(|n_threads| {
        let mut total_time = 0;
        for _ in 0..times_to_run {
            total_time += time_exec(iters, n_threads)
        }
        (total_time/times_to_run,n_threads)
    }).collect::<Vec<(u64, u64)>>();
    let time = time::precise_time_s();
    let out_name = format!("{}{}.csv", filename, time);
    let mut out = File::create(out_name).expect("Couldn't open output file");

    for (time, n_threads) in results {
        out.write(format!("{},{}\n", n_threads, time).as_bytes()).expect("Coldn't write to output file");
    }
    
}


fn time_exec(iters: u64, threads: u64) -> u64 {
    let start = time::precise_time_ns();
    let result = get_min_max_par(iters, threads);
    let end = time::precise_time_ns();
    println!("Min: {} Max: {} for {} iters in {} ms, on {} threads", result.0, result.1, iters, (end-start)/1000000, threads);
    return end-start;
}

fn main() {
    let n = 10_000_000;
    generate_perf_data(n, 100, 24, "100_24".to_string());
}
