use std::cmp;
use std::collections::HashMap;

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let k = k as usize;
        
        if n < 2 {
            return 0;
        }
        if k > n / 2 {
            let mut ans = 0;
            for i in 1..n {
                if prices[i] > prices[i - 1] {
                    ans += prices[i] - prices[i - 1];
                }
            }
            return ans;
        }
        
        let mut memory = HashMap::new();
        return Self::rec(&prices, 1, k, false, &mut memory);
    }
    
    
    fn rec(
        prices: &Vec<i32>, 
        index: usize, 
        k: usize, 
        on_purchase: bool,
        memory: &mut HashMap<(usize, usize, bool), i32>
    ) -> i32 {
        if index >= prices.len() || k == 0 {
            return 0;
        }
        if let Some(&ans) = memory.get(&(index, k, on_purchase)) {
            return ans;
        }
        let ans = cmp::max(
            prices[index] - prices[index - 1] + Self::rec(prices, index + 1, k, true, memory), 
            Self::rec(prices, index + 1, if on_purchase {k - 1} else { k }, false, memory),
        );
        memory.insert((index, k, on_purchase), ans);
        return ans;
        
    }
}
