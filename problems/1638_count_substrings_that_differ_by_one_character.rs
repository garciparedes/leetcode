use std::collections::HashMap;

impl Solution {
    pub fn count_substrings(s: String, t: String) -> i32 {
        let mut ans = 0;
        
        for i in 0..s.len() {
            ans += Self::helper(&s, &t, i, 0);
        }
        
        for j in 1..t.len() {
            ans += Self::helper(&s, &t, 0, j);
        }
        
        return ans;
    }
    
    
    fn helper(s: &str, t: &str, i: usize, j: usize) -> i32 {
        let (n, m) = (s.len(), t.len());
        let (mut ans, mut pre, mut cur) = (0, 0, 0);
        for (ii, jj) in (i..n).zip(j..m) {
            cur += 1;
            if s.chars().nth(ii) != t.chars().nth(jj) {
                pre = cur;
                cur = 0;
            }
            ans += pre;
        }
        return ans;
    }
}
