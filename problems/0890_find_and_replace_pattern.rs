use std::collections::HashMap;

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let mut mapper = HashMap::new();
        
        return words
            .into_iter()
            .filter(|word| {
                mapper.clear();    
                
                if word.len() != pattern.len() {
                    return false;
                }
                
                for (a, b) in word.chars().zip(pattern.chars()) {
                    if !mapper.contains_key(&a) {
                        if mapper.values().any(|&x| x == b) {
                            return false;
                        }
                        mapper.insert(a, b);
                    } else if mapper[&a] != b {
                        return false;
                    }
                }
                
                return true;
            })
            .collect();
        }
}
