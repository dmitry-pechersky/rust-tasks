use std::thread;
use std::sync::atomic::{compiler_fence, Ordering};

static mut X: i32 = 0;
static mut Y: i32 = 0;
static mut R1: i32 = 0;
static mut R2: i32 = 0;

fn main() {
    for i in 0.. {
        unsafe {
            X = 0;
            Y = 0;
            R1 = 0;
            R2 = 0;
        }
        let thread1 = thread::spawn(|| 
            {
                unsafe {
                    X = 1;
                    compiler_fence(Ordering::SeqCst);
                    R1 = Y;
                }
            }
        );
        let thread2 = thread::spawn(|| 
            {
                unsafe {
                    Y = 1;
                    compiler_fence(Ordering::SeqCst);
                    R2 = X;
                }
            }
        );
        thread1.join().unwrap();
        thread2.join().unwrap();
        unsafe {
            if R1 == 0 && R2 == 0 {
                println!("Broken CPU iteration {}", i);
                break;
            }
        }
    }
}
