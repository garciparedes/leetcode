use std::collections::HashMap;

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let mut memory = HashMap::new();
        Self::rec(n, true, &mut memory)
    }
    
    fn rec(n: i32, turn: bool, memory: &mut HashMap<i32, bool>) -> bool {
        if n == 0 {
            return !turn;
        }
        if let Some(&v) = memory.get(&n) {
            return !(v ^ turn);
        }
        
        let mut ans = !turn;
        let mut k = (n as f64).sqrt().floor() as i32;
        while k > 0 {
            let tmp = Self::rec(n - k.pow(2), !turn, memory);
            if tmp == turn {
                ans = tmp;
                break;
            }
            k -= 1;
        }
        memory.insert(n, turn == ans);
        return ans;
    }
}
