use std::collections::HashSet;

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            let mut uniques = HashSet::new();
            for j in i..nums.len() {
                uniques.insert(nums[j]);
                ans += uniques.len().pow(2);
            }
        }
        ans as i32
    }
}
