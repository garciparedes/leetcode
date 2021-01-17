use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut counter = HashMap::new();
        for rect in rectangles {
            *counter.entry(cmp::min(rect[0], rect[1])).or_insert(0) += 1;
        }
        
        return counter
            .into_iter()
            .max_by_key(|(k, _)| *k)
            .unwrap_or((0, 0)).1;
    }
}
