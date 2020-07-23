use std::collections::HashSet;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut memory = HashSet::new();
        for num in nums {
            if !memory.insert(num) {
                memory.remove(&num);
            } 
        }
        return memory
            .into_iter()
            .collect();
    }
}
