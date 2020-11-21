use std::collections::HashSet;

impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let num = Self::to_digits(n);
        let digits = Self::parse_digits(digits); 
        let m = num.len();
        
        let mut ans = 0;
        for i in 0..m - 1 {
            ans += digits.len().pow((i + 1) as u32);
        }
        
        ans += Self::helper(&digits, &num);
        
        return ans as i32;
    }
    
    fn parse_digits(digits: Vec<String>) -> Vec<u8> {
        let mut ans: Vec<_> = digits
            .into_iter()
            .map(|v| v.parse::<u8>().unwrap())
            .collect();
        ans.sort_unstable();
        return ans;
    }
    
    fn to_digits(mut n: i32) -> Vec<u8> {
        let mut ans = Vec::new();
        
        while n != 0 {
            ans.push((n % 10) as u8);
            n /= 10;
        }
        
        return ans
            .into_iter()
            .rev()
            .collect();
    }
    
    fn helper(digits: &[u8], num: &[u8]) -> usize {
        let n = digits.len();
        let m = num.len();
        
        if m == 0 {
            return 1;
        }
        
        let mut ans = 0;
        
        let mut i = 0;
        while i < n && digits[i] < num[0] {
            ans += n.pow((m - 1) as u32);
            i += 1;
        }
        
        if i < n && digits[i] == num[0] {
            ans += Self::helper(digits, &num[1..]);
        }
        
        return ans;
        
    }
}
