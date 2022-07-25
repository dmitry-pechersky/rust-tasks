use core::hint;
use std::{sync::atomic::{AtomicBool, Ordering}, ops::{Deref, DerefMut}, cell::UnsafeCell, thread};

pub struct SpinLock<T> { 
    atomic: AtomicBool,
    value: UnsafeCell<T>,
}

pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

struct Backoff { cnt: u32 }

impl<T> SpinLock<T> {
    pub fn new(value: T) -> Self {
        SpinLock { atomic: AtomicBool::new(false), value: UnsafeCell::new(value) }
    }

    pub fn lock(&self) -> SpinLockGuard<T> {
        self.innner_lock();
        SpinLockGuard { lock:  self }
    }

    fn innner_lock(&self) {
        let mut backoff = Backoff::new();
        while self.atomic.swap(true, Ordering::AcqRel) {
            backoff.backoff();
        }
    }

    fn inner_unlock(&self) {
        self.atomic.store(false, Ordering::Release);
    }
}

unsafe impl<T: Send> Send for SpinLock<T> {}

unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.inner_unlock();
    }
}

impl<'a, T>  Deref for SpinLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<'a, T> DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl Backoff {
    const SPIN_LIMIT: u32 = 6;

    fn new() -> Self {
        Backoff { cnt: 0 }
    }

    fn backoff(&mut self) {
        if self.cnt <= Self::SPIN_LIMIT {
            for _ in 0..(1 << self.cnt) {
                hint::spin_loop();
            }
            self.cnt += 1;
        } else {
            thread::yield_now();
        }
    }    
}
