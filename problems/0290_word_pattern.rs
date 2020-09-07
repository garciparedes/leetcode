use std::collections::{
    HashMap,
    HashSet,
};

impl Solution {
    pub fn word_pattern(pattern: String, str: String) -> bool {
        let mut matcher = HashMap::new();
        let mut unavailable = HashSet::new();
        let words: Vec<_> = str.split_whitespace().collect();
        
        if pattern.len() != words.len() {
            return false;
        }
        
        for (label, word) in pattern.chars().zip(words) {
            if let Some(p) = matcher.get(&label) {
                if *p != word {
                    return false;
                }
            } else if unavailable.contains(&word) {
                return false;
            } else {
                matcher.insert(label, word);
                unavailable.insert(word);
            }
        }
        
        return true;
    }
}
