use std::collections::HashMap;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut counter = HashMap::new();
        for c in text.chars() {
            *counter.entry(c).or_insert(0) += 1;
        }
        
        return vec![
            *counter.get(&'b').unwrap_or(&0),
            *counter.get(&'a').unwrap_or(&0),
            *counter.get(&'l').unwrap_or(&0) / 2,
            *counter.get(&'o').unwrap_or(&0) / 2,
            *counter.get(&'n').unwrap_or(&0),
        ].into_iter().min().unwrap_or(0);
    }
}
