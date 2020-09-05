use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut next_greater = HashMap::new();
        for num in nums2 {
            while let Some(other) = stack.last() {
                if *other >= num {
                    break;
                }
                next_greater.insert(stack.pop().unwrap(), num);
            }
            stack.push(num);
        }
        
        return nums1
            .iter()
            .map(|num| *next_greater.get(num).unwrap_or(&-1))
            .collect();
    }
}
