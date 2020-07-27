use std::collections::HashSet;

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut memory = HashSet::new();
        
        let mut result = HashSet::new();
        for number in nums {
            if !memory.insert(number) {
                result.insert(number);
            }
        }
        return result.into_iter().collect();
    }
}
