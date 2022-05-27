use std::sync::atomic::{ AtomicBool, Ordering };
use std::ops::{Deref, DerefMut};
use std::cell::UnsafeCell;
use std::thread;
use core::hint;

pub struct SpinLock<T> {
    atomic: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for SpinLock<T> {}
unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub fn new(value: T) -> Self {
        SpinLock { atomic: AtomicBool::new(true), value: UnsafeCell::new(value) }
    }

    pub fn lock(&self) -> SpinLockGuard<T> {
        self.inner_lock();
        SpinLockGuard::new(self)
    }

    fn inner_lock(&self) {
        const ITERATIONS_BEFORE_YIELD: usize = 1000;
        let mut iterations = 0;
        while ! self.atomic.fetch_and(false, Ordering::SeqCst) {
            if iterations < ITERATIONS_BEFORE_YIELD {
                hint::spin_loop();
            } else {
                thread::yield_now();
            }
            iterations += 1;
        }
    }

    fn inner_unlock(&self) {
        self.atomic.store(true, Ordering::Relaxed);
    }
}

pub struct SpinLockGuard<'a, T> {
    spin_lock: &'a SpinLock<T>,
}

impl<'a, T> SpinLockGuard<'a, T> {
    fn new(spin_lock: &'a SpinLock<T>) -> Self {
        SpinLockGuard { spin_lock }
    }
}

impl<'a, T>  Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.spin_lock.inner_unlock();
    }
}

impl<'a, T> Deref for SpinLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.spin_lock.value.get() }
    }
}

impl<'a, T> DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.spin_lock.value.get() }
    }
}
