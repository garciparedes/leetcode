impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut stack = Vec::new();
        let mut result = vec![-1; nums.len()];
        
        for i in 0..n * 2 {
            while !stack.is_empty() && nums[stack[stack.len() - 1]] < nums[i % n] {
                result[stack.pop().unwrap()] = nums[i % n];
            }
            stack.push(i % n);
        }
        
        return result;
    }
}
