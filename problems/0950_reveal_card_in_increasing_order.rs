use std::collections::VecDeque;

impl Solution {
    pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        let n = deck.len();
        let mut deck = deck.clone();
        deck.sort();
            
        let mut queue: VecDeque<usize> = (0..n).collect();
    
        let mut result = vec![0; n];
        for i in (0..n) {
            result[queue.pop_front().unwrap()] = deck[i];
            match queue.pop_front() {
                Some(item) => queue.push_back(item),
                _ => (),
            };
        }
        
        return result;
    }
}
