use std::cmp;
use std::collections::{
    HashMap,
};

impl Solution {
    pub fn max_freq(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32 {
        let max_letters = max_letters as usize;
        let min_size = min_size as usize;
        let max_size = max_size as usize;

        let characters: Vec<char> = s.chars().collect();
        let mut character_counter: HashMap<char, usize> = HashMap::new();
        for &c in characters[0..cmp::min(min_size, characters.len())].iter() {
            *character_counter.entry(c).or_insert(0) += 1;
        }

        let mut words: HashMap<String, i32> = HashMap::new();
        for l in 0..(characters.len() - min_size + 1) {
            let mut current_character_counter = character_counter.clone();
            for k in min_size..(max_size + 1) {
                let r = l + k - 1;
                if !(r < s.len()) {
                    break;
                }
                *current_character_counter.entry(characters[r]).or_insert(0) += 1;
                if current_character_counter.len() > max_letters {
                    break;
                }
                *words.entry(s[l..(r + 1)].to_string()).or_insert(0) += 1;
            }
            *character_counter.entry(characters[l]).or_insert(0) -= 1;
            if *character_counter.entry(characters[l] ).or_insert(0) == 0 {
                character_counter.remove(&characters[l]);
            }
            if l + min_size < characters.len() {
                *character_counter.entry(characters[l + min_size]).or_insert(0) += 1;
            }
        }
        if words.len() == 0 {
            return 0;
        }
        return *words.values().max().unwrap();
    }
}
