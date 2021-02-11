impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut i0 = 0;
        while i0 < nums.len() - 1 && nums[i0] <= nums[i0 + 1] {
            i0 += 1;
        }
        i0 += 1;
        for i in 0..nums.len() - 1 {
            if nums[(i0 + i) % nums.len()] > nums[(i0 + i + 1) % nums.len()] {
                return false;
            }
        }
        return true;
    }
}
