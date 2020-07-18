use std::collections::HashMap;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let available = Self::character_frequencies(&chars);
        
        let mut count = 0;
        for word in words.into_iter() {
            let current = Self::character_frequencies(&word);
            let mut good = false;
            for (character, count) in current.into_iter() {
                good = match available.get(&character) {
                    Some(&c) => c >= count,
                    None => false,
                };
                if !good {
                    break;
                }
            } 
            if !good {
                continue;
            }
            count += word.len();
        }
        return count as i32;
    }
    
    fn character_frequencies(word: &String) -> HashMap<char, usize> {
        let mut counter = HashMap::new();
        for c in word.chars() {
            *counter.entry(c).or_insert(0) += 1;
        }
        return counter;
    }
}
