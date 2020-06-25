impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut total: i32 = 0;
        for w in prices.windows(2) {
            if w[0] < w[1] {
                total += w[1] - w[0];
            }
        }
        return total;
    }
}
