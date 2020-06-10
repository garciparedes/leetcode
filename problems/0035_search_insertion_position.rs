impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        for (i, &num) in nums.iter().enumerate() {
            if num < target {
                continue;
            }
            return i as i32;
        }
        return nums.len() as i32;
    }
}
