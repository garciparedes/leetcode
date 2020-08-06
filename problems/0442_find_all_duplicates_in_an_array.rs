impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        for i in 0..nums.len() {
            let v = nums[i].abs();
            if nums[v as usize - 1] > 0 {
                nums[v as usize - 1] *= -1;
            } else {
                result.push(v);
            }
        }
        return result;
    }
}
