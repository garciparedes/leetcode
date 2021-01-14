use std::collections::HashMap;

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let mut counter = HashMap::new();
        for c in s.chars() {
            *counter.entry(c).or_insert(0) += 1;
        }
        
        let mut odd = 0;
        for count in counter.values() {
            if count % 2 == 1 {
                odd += 1;
            }
        }
        
        return s.len() as i32 >= k && odd <= k;
    }
}
