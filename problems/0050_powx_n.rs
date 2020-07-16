impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        let half = Self::my_pow(x, n / 2);
        
        if n % 2 == 0 {
            return half * half;
        }
        if n > 0 {
            return half * half * x;
        } else {
            return half * half / x;
        }
    }
}
