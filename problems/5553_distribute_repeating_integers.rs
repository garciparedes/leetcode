use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn can_distribute(nums: Vec<i32>, mut quantity: Vec<i32>) -> bool {
        quantity.sort_unstable_by_key(|x| -x);

        let mut counter = HashMap::new();
        for i in 0..nums.len() {
            *counter.entry(nums[i]).or_insert(0) += 1;
        }

        let mut counts: Vec<i32> = counter.values().cloned().collect();
        counts.sort_unstable_by_key(|x| -x);
        let k = cmp::min(counts.len(), 10);
        
 
        
        return Self::rec(&quantity, 0, &mut counts[..k]);
    }
    
    fn rec(quantity: &[i32], i: usize, counts: &mut [i32]) -> bool {
        if i == quantity.len() {
            return true;
        }
        
        for j in 0..counts.len() {
            if counts[j] < quantity[i] {
                continue;
            }
            counts[j] -= quantity[i];
            
            if Self::rec(quantity, i + 1, counts) {
                return true;
            }
            counts[j] += quantity[i];
        }
        return false;
        
        
    }
}
