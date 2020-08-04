use std::collections::HashMap;

impl Solution {
    pub fn custom_sort_string(s: String, t: String) -> String {
        let score: HashMap<char, usize> = s.chars().enumerate().map(|(i, x)| (x, i)).collect();
        
        let mut t: Vec<char> = t.chars().collect();
        t.sort_unstable_by_key(|x| *score.get(x).unwrap_or(&s.len()));
        
        return t.into_iter().collect();
    }
}
