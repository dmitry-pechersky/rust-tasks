use flatmap::FlatMap;

#[test]
fn basics() {
    const CAPACITY: usize = 3;
    let mut flatmap = FlatMap::<usize, usize, CAPACITY>::new();
    assert_eq!(flatmap.len(), 0);
    assert_eq!(flatmap.capacity(), CAPACITY) ;
    assert!(flatmap.is_empty()) ;
    assert_eq!(flatmap.insert(1, 10), Ok(None));
    assert_eq!(flatmap.insert(2, 20), Ok(None));
    assert_eq!(flatmap.insert(3, 30), Ok(None));
    assert_eq!(flatmap.insert(1, 100), Ok(Some(10)));
    assert_eq!(flatmap.get(&1), Some(&100));
    assert_eq!(flatmap.insert(4, 40), Err((4, 40)));
    assert_eq!(flatmap.get(&4), None );
    assert_eq!(flatmap.len(), CAPACITY);
    assert!(!flatmap.is_empty());
    assert_eq!(Some(100), flatmap.remove(&1));
    assert_eq!(Some((2,20)), flatmap.remove_entry(&2));
    assert_eq!(None, flatmap.remove_entry(&2));
    assert_eq!(flatmap.len(), 1);
    assert_eq!(Some(30), flatmap.remove(&3));
    assert_eq!(flatmap.len(), 0);
    assert!(flatmap.is_empty()) ;
}

#[test]
fn str() {
    const CAPACITY: usize = 3;
    let mut flatmap = FlatMap::<_, _, CAPACITY>::new();
    assert_eq!(flatmap.insert("one", 1), Ok(None));
    assert_eq!(flatmap.insert("two", 2), Ok(None));
    assert_eq!(flatmap.insert("three", 3), Ok(None));
    assert_eq!(flatmap.insert("four", 4), Err(("four", 4)));
    assert_eq!(flatmap.get(&"one"), Some(&1));
}