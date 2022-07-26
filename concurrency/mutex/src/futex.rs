use libc;
use std::{ptr::null, sync::atomic::AtomicU32};

pub trait Futex {
    fn wait(&self, old_value: u32);
    fn notify_one(&self);
    fn notify_all(&self);
}

impl Futex for AtomicU32 {
    fn wait(&self, old_value: u32) {
        futex_wait(&self, old_value).unwrap();
    }
    
    fn notify_one(&self) {
        futex_wake(self, 1).unwrap();
    }

    fn notify_all(&self) {
        futex_wake(self, libc::INT_MAX).unwrap();
    }
}

fn futex_wait(atomic: &AtomicU32, expected_value: u32) -> Result<(), i32> {
    let res = unsafe {
        libc::syscall(
            libc::SYS_futex, 
            atomic, 
            libc::FUTEX_WAIT + libc::FUTEX_PRIVATE_FLAG, 
            expected_value, 
            null() as *const libc::timespec, 
            null() as *const AtomicU32, 
            0
        )
    };
    if res == -1 { 
        Err(
            unsafe { *libc::__errno_location() }
        ) 
    }  else { 
        Ok(()) 
    }
}

fn futex_wake(atomic: &AtomicU32, n: i32) -> Result<i64, i32> {
    let res = unsafe {
        libc::syscall(
            libc::SYS_futex, 
            atomic, 
            libc::FUTEX_WAKE + libc::FUTEX_PRIVATE_FLAG, 
            n, 
            null() as *const libc::timespec, 
            null() as *const AtomicU32, 
            0
        )
    };
    if res == -1 { 
        Err(
            unsafe { *libc::__errno_location() }
        ) 
    }  else { 
        Ok(res) 
    }
}
