impl Solution {
    pub fn champagne_tower(mut poured: i32, query_row: i32, mut query_glass: i32) -> f64 {
        let mut dp = vec![0.0; (query_row + 1) as usize];
        dp[0] = poured as f64;
        for i in 1..(query_row + 1) as usize {
            for j in (0..i).rev() {
                dp[j] = f64::max(0.0, (dp[j] - 1.0) / 2.0);
                dp[j + 1] += dp[j];
            }
        }
        return f64::min(dp[query_glass as usize], 1.0);
    }
}
