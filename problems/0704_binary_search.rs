impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 1 && nums[0] == target {
            return 0;
        }

        let (mut i, mut j) = (0, nums.len());
        let mut k = (i + j) / 2;
        while i != k && j != k {
            if nums[k] < target {
                i = k;
            } else if nums[k] > target {
                j = k;
            }
            k = (i + j) / 2;
            if nums[k] == target {
                return k as i32;
            }
        }
        return -1;
    }
}
