use std::cmp;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 { return nums[0]; }
        return cmp::max(Self::full(&nums[0..n - 1]), Self::full(&nums[1..n]));
    }   
    
    fn full(nums: &[i32]) -> i32 {
        let (mut prev_no, mut prev_yes) = (0, 0);
        for num in nums {
            let tmp = prev_no;
            prev_no = cmp::max(prev_no, prev_yes);
            prev_yes = tmp + num;
        }
        return cmp::max(prev_yes, prev_no);
    }
}
