use std::cmp;

impl Solution {
    pub fn min_start_value(mut nums: Vec<i32>) -> i32 {
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        let minimum = nums.into_iter().fold(i32::max_value(), |acc, x| cmp::min(acc, x));
    
        return cmp::max(1, 1 - minimum);
    }
}
