impl Solution {
    pub fn min_subsequence(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by_key(|x| -x);
        let mut remaining: i32 = nums.iter().sum();
        let mut i = 0;
        let mut current = 0;
        while i < nums.len() && remaining >= current  {
            current += nums[i];
            remaining -= nums[i];
            i += 1;
        }
        return nums[..i].to_vec()
    }
}
