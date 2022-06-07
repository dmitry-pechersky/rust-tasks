use std::sync::atomic::{AtomicU64, Ordering};

pub struct Counter {
    cnt: AtomicU64,
}

impl Counter {
    pub fn new() -> Self{
        Self { cnt: AtomicU64::new(0) }
    }

    pub fn increment(&self) {
        self.cnt.fetch_add(1, Ordering::SeqCst);
    }
    
    pub fn get(&self) -> u64 {
        self.cnt.load(Ordering::SeqCst)
    }
}

#[repr(align(64))]
pub struct CounterShard {
    cnt: AtomicU64,
}

impl CounterShard {
    pub fn increment(&self) {
        self.cnt.fetch_add(1, Ordering::SeqCst);
    }
    pub fn get(&self) -> u64 {
        self.cnt.load(Ordering::SeqCst)
    }
}

pub struct ShardedCounter {
    shards: Vec<CounterShard>,
}

impl ShardedCounter {
    pub fn new(shard_cnt: usize) -> Self {
        let shards = (0..shard_cnt).into_iter().map(|_| CounterShard { cnt: AtomicU64::new(0) } ).collect::<Vec<_>>();
        Self { shards }
    }

    pub fn shard(&self, idx: usize) -> &CounterShard {
        &self.shards[idx]
    }

    pub fn get(&self) -> u64 {
        self.shards.iter().map(|shard| shard.get()).sum()
    }
}
