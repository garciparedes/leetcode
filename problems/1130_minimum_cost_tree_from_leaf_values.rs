use std::cmp;

impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut stack = vec![i32::max_value()];
        
        for item in arr {
            while stack[stack.len() - 1] <= item {
                let mid = stack.pop().unwrap();
                result += mid * cmp::min(stack[stack.len() - 1], item);
            }
            stack.push(item);
        }
        
        while stack.len() > 2 {
            result += stack.pop().unwrap() * stack[stack.len() - 1];
        }
        
        return result;
    }
}
