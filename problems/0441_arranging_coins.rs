impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let mut k = f64::sqrt(2.0 * n as f64).round() as i64;
        let sum = k * (k + 1) / 2;
        if sum > n as i64 {
            k -= 1;
        } 
        return k as i32;
    }
}
