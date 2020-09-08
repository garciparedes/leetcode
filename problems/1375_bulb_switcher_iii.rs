use std::cmp;

impl Solution {
    pub fn num_times_all_blue(light: Vec<i32>) -> i32 {
        let mut ans = 0;
        let n = light.len();
        let mut right = -1;
        for i in 0..n {
            right = cmp::max(right, light[i]);
            ans += (right == (i + 1) as i32) as i32;
        }
        return ans;
    }
}
