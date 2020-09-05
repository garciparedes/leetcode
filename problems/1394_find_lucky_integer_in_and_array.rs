use std::collections::HashMap;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut counter = HashMap::new();
        for item in arr {
            *counter.entry(item).or_insert(0) += 1;
        }
        
        return counter
            .into_iter()
            .filter(|(item, count)| item == count)
            .map(|(item, count)| item)
            .max()
            .unwrap_or(-1);
    }
}
