#![feature(test)]
extern crate test;

use sharded_counter::{Counter, ShardedCounter};
use std::{thread, sync::Arc};

const ITERATION_N: usize = 10000000;
const THREAD_N: usize = 6;

fn count(thread_cnt: usize, n: usize) -> u64 { 
    let counter = Arc::new(Counter::new());
    (0..thread_cnt).map( |_| { 
        let counter = counter.clone();
        thread::spawn( move || {
            let counter = &*counter;
            for _ in 0..n {
                counter.increment();
            }
        })
    }).collect::<Vec<_>>()
    .into_iter()
    .for_each(|handle| { handle.join().unwrap(); });
    counter.get()
}

fn sharded_count(thread_cnt: usize, n: usize) -> u64 {
    let counter = Arc::new(ShardedCounter::new(thread_cnt));
    (0..thread_cnt).map(
        |thread_i| 
        {
            let counter = counter.clone();
            thread::spawn( 
                move ||  
                {
                    let counter = counter.shard(thread_i);
                    for _ in 0..n {
                        counter.increment();
                    }
                }
            )
        }).collect::<Vec<_>>()
        .into_iter()
        .for_each(|handle| { handle.join().unwrap() });
        counter.get()
}

#[bench]
fn bench_counter(bencher: &mut test::Bencher) {
    bencher.iter( || assert_eq!((THREAD_N * ITERATION_N) as u64, count(THREAD_N, ITERATION_N)));
}

#[bench]
fn bench_sharded_counter(bencher: &mut test::Bencher) {
    bencher.iter( || assert_eq!((THREAD_N * ITERATION_N) as u64, sharded_count(THREAD_N, ITERATION_N)));
}