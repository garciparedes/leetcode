impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut decomposed: Vec<i32> = Vec::new();
        
        let mut current = n;
        while current != 0 {
            decomposed.push(current % 10);
            current /= 10;
        }

        return decomposed.iter().product::<i32>() - decomposed.iter().sum::<i32>();
    }
}
