use oneshot_channel::{ValueBuffer};
use std::{thread, sync::Arc};

#[test]
fn simple_test() {
    let buffer = ValueBuffer::new();
    assert_eq!(None, buffer.try_recv());
    buffer.send(111);
    assert_eq!(Some(111), buffer.try_recv());
}

#[test]
fn multithread_test() {
    for i in 0..100000 {
        let sender_buffer = Arc::new(ValueBuffer::new());    
        let receiver_buffer = sender_buffer.clone();
        let receiver_thread = thread::spawn( 
            move || {
                loop {
                    if let Some(value) = receiver_buffer.try_recv() {
                        return value
                    }
                }
            }
        );
        let sender_thread = thread::spawn(
            move || {
               sender_buffer.send(i);
            }
        );
        sender_thread.join().unwrap();
        assert_eq!(i , receiver_thread.join().unwrap());
    }
}
