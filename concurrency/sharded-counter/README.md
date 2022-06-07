# Sharded counter

- Counter - atomic counter
- ShardedCounter - sharded atomic counter. 

Alignment 64 to solve cache line false sharing

```
#[repr(align(64))]
pub struct CounterShard {
    cnt: AtomicU64,
}
```

## Benchmark

Counting to 10000000 in 6 threads 

```
$ cargo bench
running 2 tests
test bench_counter         ... bench: 542,681,701 ns/iter (+/- 13,552,734)
test bench_sharded_counter ... bench:  63,964,887 ns/iter (+/- 23,094,115)
```



