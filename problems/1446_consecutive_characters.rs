use std::cmp;

impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut iter = s.chars();
        
        let mut ans = 0;
        let mut curr = 1;
        let mut c1 = iter.next().unwrap();
        for c2 in iter {
            if c1 == c2 {
                curr += 1;
            } else {
                ans = cmp::max(ans, curr);
                curr = 1;
                c1 = c2;
            }
        }
        return cmp::max(ans, curr);
    }
}
