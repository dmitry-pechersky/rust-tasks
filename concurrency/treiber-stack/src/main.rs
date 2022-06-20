use treiber_stack::TreiberStack;
use std::thread;
use std::sync::Arc;

fn main() {
    let stack = Arc::new(TreiberStack::<i32>::new());
    let handlers = (0..10).into_iter().map(
        |_| 
        {
            let stack = stack.clone();
            thread::spawn( 
                move || 
                {
                    for i in 0..100000000 {
                        stack.push(i);
                        stack.pop();
                    }
                }
            )
        }
    ).collect::<Vec<_>>();
    handlers.into_iter().for_each(|handler| { handler.join().unwrap(); } );
}