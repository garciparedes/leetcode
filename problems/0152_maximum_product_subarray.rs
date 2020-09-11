use std::mem;
use std::cmp;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        
        let (mut minimum, mut maximum, mut best) = (nums[0], nums[0], nums[0]);
        for i in 1..nums.len() {
            if nums[i] < 0 {
                mem::swap(&mut minimum, &mut maximum);
            }
            maximum = cmp::max(nums[i], maximum * nums[i]);
            minimum = cmp::min(nums[i], minimum * nums[i]);
            best = cmp::max(best, maximum);
        }
        return best;
    }
}
