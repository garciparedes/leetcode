impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums.clone();
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        return nums;
    }
}
