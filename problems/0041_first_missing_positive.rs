impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        nums.push(0);
        let n = nums.len() as i32;
        for i in 0..n as usize{
            if nums[i] < 0 || nums[i] >= n {
                nums[i] = 0;
            }
        }
        for i in 0..n as usize {
            let ii = (nums[i] % n) as usize;
            nums[ii] += n;
        }
        for i in 1..n as usize {
            if nums[i] / n == 0 {
                return i as i32;
            }
        }
        return n;
    }
}
