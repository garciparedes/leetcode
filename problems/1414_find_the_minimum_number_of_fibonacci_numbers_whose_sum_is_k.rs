impl Solution {
    pub fn find_min_fibonacci_numbers(mut k: i32) -> i32 {
        let mut fib = vec![1, 1];
        while k > fib[fib.len() - 1] {
            let next = fib[fib.len() - 1] + fib[fib.len() - 2];
            fib.push(next);
        }
        
        let mut ans = 0;
        let mut i  = fib.len() - 1;
        while k > 0 {
            while fib[i] > k {
                i -= 1;
            }
            k -= fib[i];
            ans += 1;
        }
        return ans;
        
    }
}
