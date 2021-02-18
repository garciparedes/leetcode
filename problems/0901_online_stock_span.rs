struct StockSpanner {
    data: Vec<(i32, usize)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {

    fn new() -> Self {
        Self { data: vec![(1, 1)] }
    }
    
    fn next(&mut self, price: i32) -> i32 {
        let mut i = self.data.len() - 1;
        while i > 0 && self.data[i].0 <= price {
            i -= self.data[i].1;
        }
        let span = self.data.len() - i;
        self.data.push((price, span));
        return span as i32;
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */
