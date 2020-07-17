use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        let mut counter = Self::count_characters(&a[0]);
    
        for word in a[1..].iter() {
            let tmp = Self::count_characters(word);
            
            for character in counter.keys().cloned().collect::<Vec<_>>() {
                match tmp.get(&character) {
                    Some(&count) => counter.insert(character, cmp::min(counter[&character], count)),
                    None => counter.remove(&character),
                }   
            }
        }
        
        let mut result = Vec::new();
        for (character, count) in counter {
            for _ in 0..count {
                result.push(character.to_string());
            }
        }
        return result;
        
    }
    
    fn count_characters(word: &String) -> HashMap<char, usize> { 
        let mut counter = HashMap::new();
        for c in word.chars() {
            *counter.entry(c).or_insert(0) += 1;
        }
        return counter;
    }
}
