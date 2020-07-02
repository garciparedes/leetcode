use std::cmp;

impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        
        let mut result = 0;
        for w in nums.as_slice().chunks(2) {
            result += w.into_iter().min().unwrap();
        }
        return result;
    }
}
