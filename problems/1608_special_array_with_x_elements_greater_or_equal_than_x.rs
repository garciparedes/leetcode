impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort();
        println!("{:?}", nums);
        let mut j = n - 1;
        for i in (1..n + 1).rev() {
            while j > 0 && i <= nums[j - 1] as usize {
                 j -= 1;
            }
            if i <= nums[j] as usize && n - j == i {
                return i as i32;
            }
        }
        return -1;
    }
}
