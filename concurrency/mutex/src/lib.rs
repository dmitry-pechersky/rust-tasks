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
        while 1 == self.atomic.swap(1, Ordering::AcqRel) {
            self.atomic.wait(1);
        }
    }

    fn inner_unlock(&self) {
        self.atomic.store(0, Ordering::Release);
        self.atomic.notify_all();
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
