use std::cmp;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut best = 0;
        let (mut i, mut k) = (0, 0);
        let mut removed = false;
        for (j, &value) in nums.iter().enumerate() {
            if value == 0  {
                if !removed {
                    removed = true;
                    k = j + 1;
                } else {
                    i = k;
                    k = j + 1;
                }
            } 
            best = cmp::max(best, (j - i) as i32);
        }
        return best;
    }
}
