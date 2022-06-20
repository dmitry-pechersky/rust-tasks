use std::thread;
use std::sync::atomic::{compiler_fence, Ordering, AtomicI32};

fn main() {}

#[test]
fn broken_cpu() {
    static mut X1: i32 = 0;
    static mut X2: i32 = 0;
    for i in 0.. {    
        unsafe {
            X1 = 0;
            X2 = 0;
        }    
        let thread1 = thread::spawn(|| 
            {
                unsafe {
                    X1 = 1;
                    compiler_fence(Ordering::SeqCst);
                    X2
                }
            }
        );
        let thread2 = thread::spawn(|| 
            {
                unsafe {
                    X2 = 1;
                    compiler_fence(Ordering::SeqCst);
                    X1
                }
            }
        );
        assert!(thread1.join().unwrap() != 0 || thread2.join().unwrap() != 0, "Broken CPU iteration {}", i);
    }
}


#[test]
fn atomic_release_acquire() {
    static X1: AtomicI32 = AtomicI32::new(0);
    static X2: AtomicI32 = AtomicI32::new(0);
    for i in 0.. {
        X1.store(0, Ordering::SeqCst);
        X2.store(0, Ordering::SeqCst);
        let thread1 = thread::spawn(
            || 
            {
                X1.store(1, Ordering::Release);
                X2.load(Ordering::Acquire)
            }
        );
        let thread2 = thread::spawn(
            || 
            {
                X2.store(1, Ordering::Release);
                X1.load(Ordering::Acquire)
            }
        );
        assert!(thread1.join().unwrap() != 0 || thread2.join().unwrap() != 0, "Broken CPU iteration {}", i);
    }
}

#[test]
fn atomic_sec_cst() {
    static X1: AtomicI32 = AtomicI32::new(0);
    static X2: AtomicI32 = AtomicI32::new(0);
    for i in 0..1000000 {
        X1.store(0, Ordering::SeqCst);
        X2.store(0, Ordering::SeqCst);
        let thread1 = thread::spawn(
            || 
            {
                X1.store(1, Ordering::SeqCst);
                X2.load(Ordering::SeqCst)
            }
        );
        let thread2 = thread::spawn(
            || 
            {
                X2.store(1, Ordering::SeqCst);
                X1.load(Ordering::SeqCst)
            }
        );
        assert!(thread1.join().unwrap() != 0 || thread2.join().unwrap() != 0);
    }
}