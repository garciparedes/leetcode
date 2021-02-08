impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; nums.len()];
        for i in 1..nums.len() {
            ans[0] += nums[i] - nums[0];
        }
        
        for i in 1..nums.len() {
            let diff = nums[i] - nums[i - 1];
            ans[i] = ans[i - 1] + diff * (2 * i as i32 - nums.len() as i32);
        }
        
        return ans;
    }
}
