use std::cmp;

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let (mut minimum, mut maximum) = (nums[0], nums[0]);
        let mut intervals = Vec::new(); 
        for i in 1..n {
            if (minimum < nums[i] && nums[i] < maximum) {
                return true;
            }
            
            if intervals.iter().any(|(a, b)| *a < nums[i] && nums[i] < *b) {
                return true;
            }
            
            if nums[i] < minimum {
                intervals.push((minimum, maximum));
                minimum = nums[i];
                maximum = nums[i];
            } else {
                maximum = cmp::max(maximum, nums[i]);    
            }
            
        }
        return false;
    }
}
