use std::collections::HashSet;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut total = 0;
        for num in nums.iter() {
            total += num;
        }
        if total % 2 == 1 {
            return false;
        }
        let mut memory = HashSet::new();
        return Self::rec(&nums, 0, total / 2, &mut memory);
    }
    
    fn rec(
        nums: &[i32], 
        i: usize,
        remaining: i32, 
        memory: &mut HashSet<(usize, i32)>,
    ) -> bool {
        if remaining == 0 {
            return true;
        }
        
        if remaining < 0 || i >= nums.len() || memory.contains(&(i, remaining)) {
            return false;
        }
        memory.insert((i, remaining));
        
        return (
            Self::rec(nums, i + 1, remaining - nums[i], memory)
            || Self::rec(nums, i + 1, remaining, memory)
        );
    }
}
