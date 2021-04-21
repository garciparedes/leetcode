impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in (1..nums.len()) {
            if nums[i - 1] >= nums[i] {
                let tmp = 1 + nums[i - 1] - nums[i];
                ans += tmp;
                nums[i] += tmp;
            }
        }
        return ans;
    }
}
