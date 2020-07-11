impl Solution {
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 4 {
            return 0;
        }
        nums.sort();
        let mut minimum = nums[n - 4] - nums[0];
        if minimum > nums[n - 3] - nums[1] {
            minimum = nums[n - 3] - nums[1];
        }
        if minimum > nums[n - 2] - nums[2] {
            minimum = nums[n - 2] - nums[2];
        }
        if minimum > nums[n - 1] - nums[3] {
            minimum = nums[n - 1] - nums[3];
        }
        return minimum;
    }
}
