# Thread pool

Thread pool implementation using crossbeam Multi-producer multi-consumer channel

## Interface

- The `.new` method accepts the number of threads.
- The `.spawn()` method takes a task, that is actually a function, and returns its handle.
- The `JoinHandle` that is returned from `.spawn()` has a `.join()` method that returns `Result<T, JoinError>`. Here `T` is the type of the value that the task returns. `JoinError` is returned if the task panics.
- The `.shutdown()` method of the thread pool waits until all current threads finish their work and terminate them.
