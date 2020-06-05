use rand::prelude::*;
use rand::thread_rng;

struct Solution {
    weights: Vec<(i32, i32)>,
    rng: ThreadRng,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(w: Vec<i32>) -> Self {
        Solution { 
            weights: w.iter().enumerate().map(|(i, &v)| (i as i32, v)).collect(), 
            rng: thread_rng(), 
        }
    }
    
    fn pick_index(&mut self) -> i32 {
        self.weights.choose_weighted(&mut self.rng, |item| item.1).unwrap().0
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(w);
 * let ret_1: i32 = obj.pick_index();
 */
