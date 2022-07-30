mod futex;

use std::{sync::atomic::{AtomicU32, Ordering}, ops::{Deref, DerefMut}, cell::UnsafeCell};
use futex::Futex;

pub struct Mutex<T> {
    atomic: AtomicU32,
    value: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    pub fn new(value: T) -> Self {
        Self { atomic: AtomicU32::new(0), value: UnsafeCell::new(value) }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        self.inner_lock();
        MutexGuard { lock: self }
    }

    fn inner_lock(&self) {
        if let Err(c) = self.atomic.compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst) {
            loop {
                if c == 2 || self.atomic.compare_exchange(1, 2, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                    self.atomic.wait(2);
                }
                if self.atomic.compare_exchange(0, 2, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                    break;
                }
            }
        }
    }

    fn inner_unlock(&self) {
        if 1 != self.atomic.swap(0, Ordering::SeqCst) {
            self.atomic.notify_one();
        }
    }
}

pub struct MutexGuard<'a, T> {
    lock: &'a Mutex<T>,
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.inner_unlock();
    }
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

unsafe impl<T> Sync for Mutex<T> {}

unsafe impl<T> Send for Mutex<T> {}
