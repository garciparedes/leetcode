use std::collections::HashMap;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut counter = HashMap::new();
        nums.into_iter()
            .for_each(|num| {
                *counter.entry(num).or_insert(0) += 1
            });
        let max_frequency = counter.values().max().unwrap();
        counter.values()
            .filter(|freq| *freq == max_frequency)
            .sum()
    }
}
