impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        for chunk in nums.chunks(2) {
            let (freq, val) = (chunk[0] as usize, chunk[1]);
            result.extend_from_slice(&vec![val; freq]);
        }       
        return result;
    }
}
