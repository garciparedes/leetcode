use std::cmp::Reverse;
use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut counter = HashMap::new();
        for num in nums {
            *counter.entry(num).or_insert(0) += 1;
        }
        let mut counts: Vec<_> = counter.into_iter().collect();
        counts.sort_unstable_by_key(|&(label, count)| (count, Reverse(label)));
        
        let mut ans = Vec::new();
        for (label, count) in counts {
            for _ in 0..count {
                ans.push(label);
            }
        }
        return ans;
    }
}
