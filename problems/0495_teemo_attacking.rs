use std::cmp;

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut ans = duration * time_series.len() as i32;
        for i in 1..time_series.len() {
            ans -= cmp::max(0, duration - (time_series[i] - time_series[i - 1]));
        }
        return ans;
    }
}
