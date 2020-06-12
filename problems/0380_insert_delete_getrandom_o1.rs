use std::collections::HashSet;
use rand::prelude::*;
use rand::thread_rng;

struct RandomizedSet {
    data: HashSet<i32>,
    rng: ThreadRng,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    /** Initialize your data structure here. */
    fn new() -> Self {
        RandomizedSet {
            data: HashSet::new(),
            rng: thread_rng(),
        }
    }
    
    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        return self.data.insert(val);
    }
    
    /** Removes a value from the set. Returns true if the set contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        return self.data.remove(&val);        
    }
    
    /** Get a random element from the set. */
    fn get_random(&mut self) -> i32 {
        return **self
            .data
            .iter()
            .collect::<Vec<&i32>>()
            .choose(&mut self.rng)
            .unwrap();
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
