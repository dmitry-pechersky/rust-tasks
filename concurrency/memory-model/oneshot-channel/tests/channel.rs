use oneshot_channel::{channel, ChannelError};
use std::thread;

fn main() {}

#[test]
fn closed_sender_test() {
    let (sender, reciever) = channel::<i32>();
    assert_eq!(Err(ChannelError::Empty), reciever.try_recv());
    drop(sender);
    assert_eq!(Err(ChannelError::Closed), reciever.try_recv());
}

#[test]
fn closed_reciever_test() {
    let (sender, receiver) = channel::<i32>();
    drop(receiver);
    assert_eq!(Err(ChannelError::Closed), sender.send(11));
}

#[test]
fn simple_test() {
    let (sender, receiver) = channel::<i32>();
    assert_eq!(Err(ChannelError::Empty), receiver.try_recv());
    assert_eq!(Ok(()), sender.send(11));
    assert_eq!(Ok(11), receiver.try_recv());
    assert_eq!(Err(ChannelError::Closed), receiver.try_recv());
}

#[test]
fn multithread_test() {
    for i in 0..100000 {
        let (sender, reciever) = channel::<i32>();
        let receiver_thread = thread::spawn( 
            move || {
                loop {
                    match reciever.try_recv() {
                        Ok(value) => { return value; }
                        Err(error) => { assert_eq!(error, ChannelError::Empty) }
                    }
                }
            }
        );
        let sender_thread = thread::spawn(
            move || {
               sender.send(i)
            }
        );
        assert_eq!(Ok(()), sender_thread.join().unwrap());
        assert_eq!(i , receiver_thread.join().unwrap());
    }
}
