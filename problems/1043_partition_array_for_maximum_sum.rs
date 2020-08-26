use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn max_sum_after_partitioning(a: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut memory = HashMap::new();
        return Self::rec(&a[..], k, &mut memory);
    }
    
    fn rec(a: &[i32], k: usize, memory: &mut HashMap<Vec<i32>, i32>) -> i32 {
        if let Some(best) = memory.get(&a.to_vec()) {
            return *best;
        }
        let mut best = 0;
        let mut maximum = 0;
        for i in 0..cmp::min(k, a.len()) {  
            maximum = cmp::max(maximum, a[i]);
            let current = maximum * (i + 1) as i32 + Self::rec(&a[i + 1..], k, memory);
            best = cmp::max(best, current);
        }
        memory.insert(a.to_vec(), best);
        return best;
    }
}
