use std::collections::{HashMap, HashSet};
use std::cmp;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let word_dict: HashSet<_> = word_dict.into_iter().collect();
        let mut longest = 0;
        for word in word_dict.iter() {
            if longest < word.len() {
                longest = word.len();
            }
        }
        let mut memory = HashMap::new();
        
        return Self::rec(&word_dict, &s, 0, longest, &mut memory);
    }
    
    fn rec(
        word_dict: &HashSet<String>, s: &String, i: usize, longest: usize, memory: &mut HashMap<usize, bool>,
    ) -> bool {
        if let Some(ans) = memory.get(&i) {
            return *ans;
        }
        if i == s.len() {
            return true;
        }
        if i > s.len() {
            return false;
        }
        for j in i + 1..cmp::min(i + longest, s.len())  + 1 {
            if !word_dict.contains(&s[i..j]) {
                continue;
            }
            if !Self::rec(word_dict, s, j, longest, memory) {
                continue;
            }
            return true;
        }
        memory.insert(i, false);
        return false;
        
    }
}
