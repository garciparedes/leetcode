use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut counter = HashMap::new();
        for c in s.chars() {
            *counter.entry(c).or_insert(0) += 1;
        }
        let mut counter: Vec<_> = counter.into_iter().collect();
        counter.sort_unstable_by_key(|(c, count)| -count);
        
        let mut ans = String::new();
        for (c, count) in counter {
            for _ in 0..count {
                ans.push(c);
            }
        }
        return ans;
    }
}
