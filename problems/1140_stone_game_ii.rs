use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let mut memory = HashMap::new();
        
        return Self::play_round(&piles[..], 1, true, &mut memory).0
    }
    
    fn play_round(
        piles: &[i32], 
        m: usize, 
        turn: bool, 
        memory: &mut HashMap<(usize, usize, bool), (i32, i32)>,
    ) -> (i32, i32) {
        if let Some(ans) = memory.get(&(piles.len(), m, turn)) {
           return *ans;
        }
        
        let mut best = (0, 0);
        for x in 1..cmp::min(piles.len(), 2 * m) + 1 {
            let mut current = Self::play_round(&piles[x..], cmp::max(m, x), !turn, memory);
            
            if turn {
                current = (current.0 + piles[..x].iter().sum::<i32>(), current.1);    
            } else {
                current = (current.0, current.1 + piles[..x].iter().sum::<i32>());    
            }
            
            if (turn && current.0 > best.0) || (!turn && current.1 > best.1) {
                best = current;
            } 
        }
        
        memory.insert((piles.len(), m, turn), best);
        return best;
    }
}
