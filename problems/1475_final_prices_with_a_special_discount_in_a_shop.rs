impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        for (i, &price) in prices.iter().enumerate() {
            let mut discount = 0;
            for j in i + 1..prices.len() {
                if prices[j] <= price {
                    discount = prices[j];
                    break;
                } 
            }
            result.push(price - discount); 
        }
        return result;
    }
}
