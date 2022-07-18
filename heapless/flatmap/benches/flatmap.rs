#![feature(test)]
extern crate test;

use flatmap::FlatMap;
use std::collections::HashMap;

const COUNT: usize = 100000;
const CAPACITY: usize = 20;

#[bench]
fn bench_flatmap(bencher: &mut test::Bencher) {
    bencher.iter(
        ||
        {
            let mut flatmap = FlatMap::<usize, usize, CAPACITY>::new();
            for _ in 0..COUNT {
                for j in 0..CAPACITY {
                    flatmap.insert(j, j);
                }
                for j in 0..CAPACITY {
                    flatmap.remove(&j);    
                }
            }        
        }
    );
}

#[bench]
fn bench_hashmap(bencher: &mut test::Bencher) {
    bencher.iter(
        || 
        { 
            let mut hashmap = HashMap::new();
            for _ in 0..COUNT {
                for j in 0..CAPACITY {
                    hashmap.insert(j, j);
                }
                for j in 0..CAPACITY {
                    hashmap.remove(&j);    
                }
            }        
        }
    );
}