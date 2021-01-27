impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s: Vec<_> = s.chars().into_iter().collect();
        let mut ans = 0;
        
        for offset in 1..=s.len() {
            for start in 0..=s.len() - offset {
                if Self::is_palindromic(&s[start..start + offset]) {
                    ans += 1;
                }
            }
        }
        
        return ans;
    }
    
    fn is_palindromic(s: &[char]) -> bool {
        for i in 0..s.len() / 2 {
            if s[i] != s[s.len() - (i + 1)] {
                return false;
            }
        }
        return true;
    }
}
