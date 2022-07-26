use mutex::Mutex;
use std::{thread, sync::Arc, time::Duration};

#[test]
fn one_mutex() {
    let mutex = Mutex::new(1);
    {    
        let mut guard = mutex.lock();
        *guard += 1;    
    }
    {    
        let mut guard = mutex.lock();
        *guard += 1;    
    }
    assert_eq!(*mutex.lock(), 3);
}

#[test]
fn two_mutexes() {
    let mutex1 = Mutex::new(1);
    let mutex2 = Mutex::new(2);
    {    
        let mut guard1 = mutex1.lock();
        *guard1 += 1;    
        let mut guard2 = mutex2.lock();
        *guard2 += 2;    
    }
    assert_eq!(*mutex1.lock(), 2);
    assert_eq!(*mutex2.lock(), 4);
}

#[test]
fn mutual_exclusion() {
    let mutex = Arc::new(Mutex::new(1));
    let thread1 = {
        let mutex = mutex.clone();
        thread::spawn(move || {
            let mut guard = mutex.lock();
            *guard = 2;
            thread::sleep(Duration::from_secs(3));
            *guard = 3;
        })        
    };
    thread::sleep(Duration::from_secs(1));
    assert_eq!(*mutex.lock(), 3);
    thread1.join().unwrap();
}