# Mutex

## Interface

- Mutex 
- MutexGuard implements Deref DerefMut traits

## Implementation

Using Linux futex for blocking

3 states:
- 0 unlocked
- 1 locked, now waiters
- 2 locked, one or more waiters


## Todo

- poisoning ?
- orderings weakening ?
