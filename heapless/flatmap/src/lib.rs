#![no_std]
use core::{mem};

pub struct FlatMap<K, V, const COUNT: usize> {
    length: usize,
    array: [Option<(K, V)>; COUNT],
}

impl<K, V, const COUNT: usize> FlatMap<K, V, COUNT> 
where K: PartialEq
{
    const INIT: Option<(K, V)> = None;

    pub fn new() -> Self {
        FlatMap { length: 0, array: [Self::INIT; COUNT]}
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.find_key_idx(key).map( |i| &self.array[i].as_ref().unwrap().1)
    }

    pub fn insert(&mut self, key: K, value: V) -> Result<Option<V>, (K, V)> {
        if let Some(i) = self.find_key_idx(&key) {
            return Ok(Some(mem::replace(&mut self.array[i].as_mut().unwrap().1, value)));            
        }

        if self.len() < self.capacity() {
            for i in 0..self.array.len() {
                if self.array[i].is_none() {
                    self.array[i] = Some((key, value));
                    self.length += 1;
                    return Ok(None)
                }
            }    
        }

        Err((key, value))
    }

    pub fn capacity(&self) -> usize {
        self.array.len()
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(idx) = self.find_key_idx(&key) {
            self.length -= 1; 
            Some(self.array[idx].take().unwrap().1)
        } else {
            None
        }
    }

    pub fn remove_entry(&mut self, key: &K) -> Option<(K, V)> {
        if let Some(idx) = self.find_key_idx(&key) {
            self.length -= 1; 
            self.array[idx].take()
        } else {
            None
        }
    }

    fn find_key_idx(&self, key: &K) -> Option<usize> {
        for i in 0..self.array.len() {
            if let Some(item) = &self.array[i] {
                if item.0 == *key {
                    return Some(i)
                }
            }
        }
        None
    }

}


