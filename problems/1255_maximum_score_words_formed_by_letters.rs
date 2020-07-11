use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {        
        let mut available_letters = HashMap::new();
        for letter in letters {
            *available_letters.entry(letter).or_insert(0) += 1;
        }
        
        return Self::rec(words, available_letters, &score)
    }
    
    fn rec(words: Vec<String>, letters: HashMap<char, usize>, score: &Vec<i32>) -> i32 {
        let mut best = 0;
        
        for (i, word) in words.iter().enumerate() {
            let mut current_letters = letters.clone();
            let mut valid = true;
            let mut current_score = 0;
            for letter in word.chars() {
                current_score += score[(letter as u32 - 97) as usize];
                
                let count = current_letters.entry(letter).or_insert(0);
                if *count < 1 {
                    valid = false;
                    break;
                }
                *count -= 1;
            }
            if !valid {
                continue;
            }
            let mut remaining_words: Vec<String> = words[..i].to_vec();
            remaining_words.extend(words[i + 1..].to_vec());
            
            current_score += Self::rec(remaining_words, current_letters, score);
            
            best = cmp::max(best, current_score);
        }
        return best;
    }
}
