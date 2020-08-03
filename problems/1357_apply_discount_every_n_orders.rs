use std::collections::HashMap;

struct Cashier {
    state: usize,
    n: usize,
    discount: f64,
    product_prices: HashMap<i32, i32>,
}

impl Cashier {

    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        Cashier {
            state: 0,
            n: n as usize,
            discount: discount as f64 / 100.0,
            product_prices: products.into_iter().zip(prices).collect::<HashMap<i32,i32>>(),
        }
    }
    
    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {        
        self.state += 1;
        
        let mut price = product
            .iter()
            .zip(amount)
            .map(|(p, a)| (self.product_prices[p] * a) as f64)
            .fold(0.0, |acc, x| acc + x);
        
        if self.state == self.n {
            self.state = 0;
            price *= (1.0 - self.discount);
        }
        
        return price;
    }
}

