impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut twos_counter = 0;
        while i < nums.len() - twos_counter{
            if nums[i] == 2 {
                nums.remove(i);
                nums.push(2);
                twos_counter += 1;
            } else if nums[i] == 0 {
                nums.remove(i);
                nums.insert(0, 0);
                i += 1;
            } else {
                i += 1;
            }
        }
    }
}
