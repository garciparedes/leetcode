impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut result = Vec::new();
        for i in 0..n {
            result.push(nums[i]);
            result.push(nums[i + n]);
        }
        return result;
    }
}
