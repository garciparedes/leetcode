impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let (mut current, mut count) = (nums[0], 0);
        let mut i = 0;
        while i < nums.len() {
            if nums[i] != current {
                current = nums[i];
                count = 0;
                continue;
            } 
            
            if count < 2 {
                count += 1;
                i += 1;
                continue;
            }
            
            nums.remove(i);
        }
        return nums.len() as i32;
    }
}
