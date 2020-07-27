impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let low_odd = (low % 2 == 1) as i32;
        let high_odd = (high % 2 == 1) as i32;
        if low == high {
            return low_odd;
        }
        (high - low - low_odd) / 2 + high_odd + low_odd
    }
}
