use std::collections::HashMap;
use std::cmp::Ordering;

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut counter = HashMap::new();
        for t in time {
            *counter.entry(t % 60).or_insert(0) += 1;
        }
        
        let mut ans = 0;
        for (&base, &a) in counter.iter().filter(|(&base, _)| base > 30) {
            if let Some(b) = counter.get(&(60 - base)) {
                ans += a * b;
            }
        }
        
        if let Some(&count) = counter.get(&30) {
            ans += Self::binom(count, 2);
        }
        if let Some(&count) = counter.get(&0) {
            ans += Self::binom(count, 2);
        }
        return ans;
    }
    
    fn binom(n: i32, k: i32) -> i32 {
        (0..n + 1)
            .rev()
            .zip(1..k + 1)
            .fold(1, |mut ans, (dividend, divisor)| {
                ans *= dividend;
                ans /= divisor;
                return ans;
            })
    }
}
