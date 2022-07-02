use oneshot_channel::RelaxedValueBuffer;
use std::{thread, sync::Arc}; 
 
 #[test]
fn relaxedvaluebuffer_test() {
    for i in 0..100000 {
        let sender_buffer = Arc::new(RelaxedValueBuffer::new());    
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