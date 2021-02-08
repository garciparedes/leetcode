use std::collections::HashMap;

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut counter = HashMap::new();
        for num in nums {
            *counter.entry(num).or_insert(0) += 1;
        }
        
        return counter
            .into_iter()
            .filter(|(_, v)| *v == 1)
            .map(|(k, _)| k)
            .sum();
    }
}
