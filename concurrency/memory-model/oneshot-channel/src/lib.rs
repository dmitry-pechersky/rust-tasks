use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

#[derive(Debug, PartialEq)]
pub enum ChannelError {
    Closed,
    Empty,
}

pub struct Receiver<T> {
    buffer: Option<Arc<ValueBuffer<T>>>,
}

pub struct Sender<T> {
    buffer: Arc<ValueBuffer<T>>,
}

pub struct ValueBuffer<T> {
    value: Option<T>,
    status: AtomicBool,
}

pub struct RelaxedValueBuffer<T> {
    value: Option<T>,
    status: AtomicBool,
}


pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let buffer = Arc::new(ValueBuffer::new());
    (Sender { buffer: buffer.clone() }, Receiver { buffer: Some(buffer) })
}

impl<T> Sender<T> {
    pub fn send(self, value: T) -> Result<(), ChannelError> {
        if Arc::strong_count(&self.buffer) > 1 {
            self.buffer.send(value);
            return Ok(());
        }
        Err(ChannelError::Closed)
    }
}

impl<T> Receiver<T> {
    pub fn  try_recv(&self) -> Result<T, ChannelError> {
        if let Some(buffer) = &self.buffer {
            let channel_open = Arc::strong_count(buffer) > 1;     
            if let Some(value) = buffer.try_recv() {
                return Ok(value);
            } else {
                if channel_open { Err(ChannelError::Empty) } else { Err(ChannelError::Closed) }
            }
        } else {
            Err(ChannelError::Closed)
        }
    }
}

impl<T> ValueBuffer<T> {
    pub fn new() -> Self {
        Self { value: None, status: AtomicBool::new(false) }
    }

    pub fn send(&self, value: T) {
        let value_ptr = (&self.value as * const Option<T>) as * mut Option<T>;
        unsafe {
            *value_ptr = Some(value);
        }
        self.status.store(true, Ordering::Release);
    }

    pub fn try_recv(&self) -> Option<T> {
        let value_ptr = (&self.value as * const Option<T>) as * mut Option<T>;
        if self.status.load(Ordering::Acquire) { 
            unsafe { (&mut *value_ptr).take() } 
        } else { 
            None 
        }
    }
}


impl<T> RelaxedValueBuffer<T> {
    pub fn new() -> Self {
        Self { value: None, status: AtomicBool::new(false) }
    }

    pub fn send(&self, value: T) {
        let value_ptr = (&self.value as * const Option<T>) as * mut Option<T>;
        unsafe {
            *value_ptr = Some(value);
        }
        self.status.store(true, Ordering::Relaxed);
    }

    pub fn try_recv(&self) -> Option<T> {
        let value_ptr = (&self.value as * const Option<T>) as * mut Option<T>;
        if self.status.load(Ordering::Relaxed) { 
            unsafe { (&mut *value_ptr).take() } 
        } else { 
            None 
        }
    }
}