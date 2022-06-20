use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;
use std::alloc::{alloc, dealloc, Layout};

struct Node<T> {
    value: T,
    next: * const Node<T>,
}

pub struct TreiberStack<T> {
    head: AtomicPtr<Node<T>>,
}

impl<T> TreiberStack<T> {
    pub fn new() -> Self {
        let head = AtomicPtr::new(ptr::null::<Node<T>>() as *mut Node<T>);
        Self { head }
    }

    pub fn push(&self, value: T) {
        let layout = Layout::new::<Node<T>>();
        let cur_head;
        unsafe {
            cur_head = alloc(layout) as *mut Node<T>;
            (*cur_head).value = value;
        }
        loop {
            unsafe {
                (*cur_head).next = self.head.load(Ordering::SeqCst);
            }
            if self.head.compare_exchange_weak(unsafe { (*cur_head).next } as *mut Node<T>, cur_head, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                break;
            }
        }
    }

    pub fn pop(&self) -> Option<T> {
        let layout = Layout::new::<Node<T>>();
        loop {
            let cur_head = self.head.load(Ordering::SeqCst);
            if cur_head as *const Node<T> == ptr::null::<Node<T>>() {
                break;
            }
            if self.head.compare_exchange_weak(cur_head as *mut Node<T>,  unsafe { (*cur_head).next as *mut Node<T> }, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                unsafe {                
                    let value = ptr::read(&(*cur_head).value);
                    dealloc(cur_head as *mut u8, layout);
                    return Some(value);
                }
            }
        }
        None
    }

    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::SeqCst) as *const Node<T> == ptr::null::<Node<T>>()
    }
}

impl<T> Drop for TreiberStack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() { }
    }
}
