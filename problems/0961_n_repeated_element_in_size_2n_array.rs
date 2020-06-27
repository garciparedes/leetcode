use std::collections::HashSet;

impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        let mut alphabet = HashSet::new();
        
        for item in a {
            if !alphabet.insert(item) {
                return item;
            }
        }
        return -1;
    }
}
