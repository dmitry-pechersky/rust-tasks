use crossbeam::channel::{ unbounded, bounded, Receiver, Sender };
use std::{ panic::{catch_unwind, AssertUnwindSafe}, thread };

fn worker(receiver: Receiver<Box<dyn FnOnce() + Send>>) {
    for task in receiver {
        task();
    }
}

pub struct ThreadPool {
    sender: Sender<Box<dyn FnOnce() + Send>>,
    thread_handlers: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(thread_cnt: usize) -> Self {
        let (sender, receiver) = unbounded();
        let thread_handlers = (0..thread_cnt).map( |_| {
                let receiver = receiver.clone();
                thread::spawn(move || { worker(receiver); })
            })
            .collect::<Vec<_>>();
        Self { sender, thread_handlers }
    }

    pub fn spawn<F, T>(&self, task: F) -> JoinHandle<T>
        where 
            F: FnOnce() -> T,
            F: Send + 'static,
            T: Send + 'static, 
    {
        let (result_sender, result_receiver) = bounded(1);
        self.sender.send(
            Box::new( 
                move || 
                { 
                    if let Ok(result) = catch_unwind(AssertUnwindSafe(task)){
                        result_sender.send(result).err();
                    }
                } 
            )
        ).unwrap();
        JoinHandle { result_receiver }
    }

    pub fn shutdown(self) {
        drop(self.sender);
        self.thread_handlers.into_iter().for_each(|handler| { handler.join().unwrap(); });
    }
}

pub struct JoinHandle<T> {
    result_receiver: Receiver<T>,
}

#[derive(Debug)]
pub struct JoinError {}

impl<T> JoinHandle<T> {
    pub fn join(self) -> Result<T, JoinError> {
        self.result_receiver.recv().map_err(|_| JoinError { })
    }
}
