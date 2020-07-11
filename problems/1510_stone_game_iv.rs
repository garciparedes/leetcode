use std::collections::HashMap;

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {   
        let mut memory = HashMap::new();
        return Self::rec(n, true, &mut memory);        
    }
    
    fn rec(n: i32, turn: bool, memory: &mut HashMap<i32, bool>) -> bool {
        if memory.contains_key(&n) {
            if memory[&n] {
                return turn;
            } else {
                return !turn;
            }
        }
        let mut exp = 1;
        while i32::pow(exp, 2) <= n {
            if turn == Self::rec(n - i32::pow(exp, 2), !turn, memory) {
                memory.insert(n, true);
                return turn;
            }
            exp += 1;
        }
        
        memory.insert(n, false);
        return !turn;
    }
}
