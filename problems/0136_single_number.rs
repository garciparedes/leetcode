use std::collections::HashSet;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut memory: HashSet<i32> = HashSet::new();
        
        for &num in nums.iter() {
            if memory.contains(&num) {
                memory.remove(&num);
            } else {
                memory.insert(num);
            }
        }
        return *memory.iter().nth(0).unwrap();
    }
}
