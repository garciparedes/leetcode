use std::collections::HashMap;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut counter = HashMap::new();
        for item in arr {
            *counter.entry(item).or_insert(0) += 1;
        }
        
        let mut counts: Vec<_> = counter.values().into_iter().collect();
        counts.sort_unstable();
        
        let mut cumulated = 0;
        let mut i = 0;
        while cumulated < n / 2 {
            cumulated += counts[counts.len() - 1 - i];
            i += 1;
        }
        
        return i as i32;
        
    }
}
