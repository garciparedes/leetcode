use std::cmp;
use std::collections::{
    HashMap,
    HashSet,
};

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 { 
        if prices.is_empty() {
            return 0;
        }
        let mut memory = HashMap::new();   
        let mut best = 0;
        for i in 0..prices.len() - 1 {
            best = cmp::max(best, Self::rec(&prices, i, false, &mut memory))
        }
        return best;
    }
    
    
    fn rec(
        prices: &Vec<i32>, 
        actual: usize, 
        parity: bool, 
        memory: &mut HashMap<(usize, bool), i32>
    ) -> i32 {
        if let Some(best) = memory.get(&(actual, parity)) {
            return *best;
        }
        let last = actual;
        let mut best = 0;
        for i in (last + 1 + parity as usize)..prices.len() {
            let mut diff = 0;
            if !parity  {   
                diff = prices[i] - prices[last];
                if diff <= 0 {
                    continue;
                }
            }
            best = cmp::max(best, diff + Self::rec(prices, i, !parity, memory))
        }
        memory.insert((actual, parity), best);
        return best;
    }
}
