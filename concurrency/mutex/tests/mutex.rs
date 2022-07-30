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

#[test]
fn mutual_exclusion_three_threads() {
    let mutex = Arc::new(Mutex::new(1));
    let thread1 = {
        let mutex = mutex.clone();
        thread::Builder::new().name("thread1".to_string()).spawn(move || {
            let mut guard = mutex.lock();
            *guard += 1;
            thread::sleep(Duration::from_secs(3));
        }).unwrap() 
    };
    let thread2 = {
        let mutex = mutex.clone();
        thread::Builder::new().name("thread2".to_string()).spawn(move || {
            thread::sleep(Duration::from_secs(1));
            let mut guard = mutex.lock();
            *guard += 1;
        }).unwrap()
    };
    thread::sleep(Duration::from_secs(2));
    assert_eq!(3, *mutex.lock());
    thread1.join().unwrap();
    thread2.join().unwrap();
}


#[test]
fn stress_test() {
    const THREAD_CNT: usize = 10000;
    const ITERATION_CNT: usize = 1000;
    let mutex = Arc::new(Mutex::new(0));
    let threads = (0.. THREAD_CNT).map(|_| {
        let mutex = mutex.clone();
        thread::spawn(move || {
            for _ in 0..ITERATION_CNT {
                let mut guard = mutex.lock();
                *guard += 1;
            }
        })
    }).collect::<Vec<_>>();
    threads.into_iter().for_each(| handle | handle.join().unwrap());
    assert_eq!(THREAD_CNT * ITERATION_CNT, *mutex.lock());
}

#[test]
fn stress_sleep_test() {
    const THREAD_CNT: usize = 1000;
    const ITERATION_CNT: usize = 10;
    let mutex = Arc::new(Mutex::new(0));
    let threads = (0.. THREAD_CNT).map(|_| {
        let mutex = mutex.clone();
        thread::spawn(move || {
            for _ in 0..ITERATION_CNT {
                let mut guard = mutex.lock();
                thread::sleep(Duration::from_millis(1));
                *guard += 1;
            }
        })
    }).collect::<Vec<_>>();
    threads.into_iter().for_each(| handle | handle.join().unwrap());
    assert_eq!(THREAD_CNT * ITERATION_CNT, *mutex.lock());
}
