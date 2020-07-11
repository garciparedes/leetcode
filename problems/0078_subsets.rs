use std::collections::HashSet;

impl Solution {
    pub fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        
        let mut result = HashSet::new();
        Self::rec(nums, &mut result);
        
        return result
            .into_iter()
            .collect();
    }
    
    fn rec(nums: Vec<i32>, result: &mut HashSet<Vec<i32>>) {
        if result.contains(&nums) {
            return;
        }
        result.insert(nums.clone());
        let n = nums.len();
        for i in 0..n {
            let mut current = nums[..i].to_vec();
            current.extend(nums[i + 1..].iter());
            Self::rec(current, result);
        }
    }
}
