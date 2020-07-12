use std::collections::HashMap;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut counter = HashMap::new();
        for num in nums {
            *counter.entry(num).or_insert(0) += 1;
        }   
        
        return counter
            .values()
            .into_iter()
            .fold(0, |acc, x| acc + x * (x - 1) / 2);
    }
}
