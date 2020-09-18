use std::cmp;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let (mut minimum, mut ans) = (prices[0], 0); 
        for price in prices {
            let current = price - minimum;
            if minimum > price {
                minimum = price;
            }
            if current > ans {
                ans = current;
            }
        }
        return ans;
    }
}
