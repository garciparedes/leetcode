use std::cmp;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        return Self::dfs(&nums, 0, nums.len() - 1);
    }
    
    fn dfs(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
        if nums[start] <= nums[end] {
            return nums[start];
        }
        let middle = start + (end - start) / 2;

        return cmp::min(
            Self::dfs(nums, start, middle),
            Self::dfs(nums, middle + 1, end),
        )
    }
}
