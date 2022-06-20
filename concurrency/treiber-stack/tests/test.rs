use treiber_stack::TreiberStack;
use std::thread;
use std::sync::Arc;

#[test]
fn simple_test() {
    let stack = TreiberStack::<i32>::new();
    assert!(stack.is_empty())
    
    ;
    assert_eq!(None, stack.pop());

    stack.push(10);
    assert!(!stack.is_empty());

    stack.push(11);
    assert!(!stack.is_empty());

    assert_eq!(Some(11), stack.pop());
    assert!(!stack.is_empty());

    assert_eq!(Some(10), stack.pop());
    assert!(stack.is_empty());

    assert_eq!(None, stack.pop());
}

#[test]
fn multithread_test() {
    let stack = Arc::new(TreiberStack::<i32>::new());
    let handlers = (0..10).into_iter().map(
        |_| 
        {
            let stack = stack.clone();
            thread::spawn( 
                move || 
                {
                    for i in 0..100000 {
                        stack.push(i);
                        stack.pop();
                    }
                }
            )
        }
    ).collect::<Vec<_>>();
    handlers.into_iter().for_each(|handler| { handler.join().unwrap(); } );
}