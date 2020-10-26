use std::cmp;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let s: Vec<_> = s.chars().collect();        
        let mut ans = -1;
        for i in 0..s.len() {
            for j in (i + 1..s.len()).rev() {
                if s[i] == s[j] {
                    ans = cmp::max(ans, (j - i) as i32 - 1);
                    break;
                }
            } 
        }
        return ans;
        
    }
}
