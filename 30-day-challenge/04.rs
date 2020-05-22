impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i: usize = 0;
        let mut moved: usize = 0;
        while i < nums.len() - moved {
            if nums[i] != 0 {
                i += 1;
                continue;
            }
            nums.remove(i);
            nums.push(0);
            moved += 1;
        }
    }
}
