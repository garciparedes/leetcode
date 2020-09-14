use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut memory = HashMap::new(); 
        return cmp::max(
            Self::dfs(&nums[..], 0, &mut memory),  
            Self::dfs(&nums[..], 1, &mut memory),
        );
    }
    
    fn dfs(nums: &[i32], index: usize, memory: &mut HashMap<usize, i32>) -> i32 {
        if index >= nums.len() {
            return 0;
        }
        if let Some(ans) = memory.get(&index) {
            return *ans;
        }
        
        let ans = nums[index] + cmp::max(
            Self::dfs(nums, index + 2, memory),
            Self::dfs(nums, index + 3, memory),
        );
        memory.insert(index, ans);
        return ans;
    }
}
