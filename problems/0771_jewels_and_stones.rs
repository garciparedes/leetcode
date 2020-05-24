use std::collections::HashSet;

impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let mut jewels = HashSet::new();        
        for jewel in j.chars() {
            jewels.insert(jewel);
        }

        let mut counter = 0;
        for stone in s.chars() {
            counter += jewels.contains(&stone) as i32;
        }
        return counter;
    }
}
