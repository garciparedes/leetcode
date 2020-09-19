use std::cmp;
use std::collections::HashMap;

impl Solution {
    pub fn minimum_distance(word: String) -> i32 {
        let word: Vec<_> = word.chars().map(|c| c as u8 - 'A' as u8).collect();
        let mut memory = HashMap::new();
        let ans =  Self::rec(&word, 1, 26, &mut memory);
        return ans;
    }
    
    fn rec(word: &Vec<u8>, pos: usize, other: u8, memory: &mut HashMap<(usize, u8), i32>) -> i32 {
        if pos >= word.len() {
            return 0;
        }
        
        if let Some(ans) = memory.get(&(pos, other)) {
            return *ans;
        }
        
        let (to, last) = (word[pos], word[pos - 1]);
        
        let ans = cmp::min(
            Self::cost(last, to) + Self::rec(word, pos + 1, other, memory),
            Self::cost(other, to) + Self::rec(word, pos + 1, last, memory),
        );
        
        memory.insert((pos, other), ans);
        return ans;
    }
    
    fn cost(from: u8, to: u8) -> i32 {
        let (from, to) = (from as i32, to as i32);
        if from == 26 {
            return 0;
        }
        return (from / 6 - to / 6).abs()  + (from % 6 - to % 6).abs();
    }
}
