# One hot channel and value buffer

ValueBuffer - value buffer with atomics and correct orderings for happens before order

channel, Receiver, Sender - one shot channel using ValueBuffer 

RelaxedValueBuffer - incorrect ordering Relaxed. Difficult to find data race in tests. But ThreadSanitizer finds it.

```
export RUSTFLAGS=-Zsanitizer=thread RUSTDOCFLAGS=-Zsanitizer=thread
cargo test --test relaxedvaluebuffer -Zbuild-std --target x86_64-unknown-linux-gnu
...
WARNING: ThreadSanitizer: data race (pid=1016486)
...
```