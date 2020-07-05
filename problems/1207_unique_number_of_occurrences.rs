use std::collections::{
    HashMap,
    HashSet,
};

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut counter = HashMap::new();
        for item in arr {
            *counter.entry(item).or_insert(0) += 1;
        }
        let distinct_counts = counter
            .values()
            .cloned()
            .collect::<HashSet<usize>>()
            .len();
        
        return counter.len() == distinct_counts;
    }
}
