use std::collections::HashMap;

impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut counter = HashMap::new();
        
        for ball in low_limit..=high_limit {
            let index = Self::index(ball);
            *counter.entry(index).or_insert(0) += 1;
        }
        
        return counter.values().max().unwrap().to_owned();
    }
    
    fn index(mut ball: i32) -> i32 {
        let mut ans = 0;
        while ball > 0 {
            ans += ball % 10;
            ball /= 10;
        }
        return ans;
    }
}
