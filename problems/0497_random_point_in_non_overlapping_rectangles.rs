use rand::prelude::*;
use rand::distributions::{Weighted, WeightedChoice, Distribution};

struct Solution {
    rectangles: Vec<Vec<i32>>,
    rng: ThreadRng,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(rects: Vec<Vec<i32>>) -> Self {
        Solution { 
            rectangles: rects, 
            rng: thread_rng(),
        }
    }
    
    fn pick(&mut self) -> Vec<i32> {
        let mut items: Vec<_> = self.rectangles
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, x)| Weighted { weight: ((x[2] - x[0] + 1) * (x[3] - x[1] + 1)) as u32, item: i })
            .collect();

        let weights = WeightedChoice::new(&mut items);

        
        let i = weights.sample(&mut self.rng);
        let rectangle = &self.rectangles[i];
        let x = self.rng.gen_range(rectangle[0], rectangle[2] + 1);
        let y = self.rng.gen_range(rectangle[1], rectangle[3] + 1);
        return vec![x, y];
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(rects);
 * let ret_1: Vec<i32> = obj.pick();
 */
