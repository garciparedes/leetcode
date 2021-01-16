use std::collections::HashSet;
use std::cmp;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {        
        cmp::min(
            candy_type.len() / 2,
            candy_type.into_iter().collect::<HashSet<_>>().len(),
        ) as i32
    }
}
