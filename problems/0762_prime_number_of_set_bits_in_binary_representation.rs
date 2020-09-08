impl Solution {
    pub fn count_prime_set_bits(l: i32, r: i32) -> i32 {
        let mut ans = 0;
        for number in l..(r + 1) {
            let s = Self::binary_set(number);       
            ans += Self::is_prime(s) as i32;
        }
        return ans;
    }
    
    fn binary_set(number: i32) -> i32 {
        let n = (number as f64).log2().ceil() as i32;
        let mut ans = 0;
        for i in 0..(n + 1) {
            ans += (((1 << i) & number) != 0) as i32;
        }
        return ans;
    }
    
    fn is_prime(number: i32) -> bool {
        if number <= 3 {
            return number > 1;
        }
        if (number % 2 == 0) || (number % 3 == 0) {
            return false;
        }
        let mut i = 5;
        
        while i * i <= number {
            if (number % i == 0) || (number % (i + 2) == 0) {
                return false;
            }
            i += 6;
        }
        return true;
    }
    
}
