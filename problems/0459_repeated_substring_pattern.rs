impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let s: Vec<_> = s.chars().collect();
    
        return (1..(s.len() / 2 + 1))
            .any(|period| Self::check_periodicity(&s, period));
    }
    
    fn check_periodicity(s: &Vec<char>, period: usize) -> bool { 
        let n = s.len();
        if n % period != 0 {
            return false;
        }
        
        for i in 1..(n / period) {
            for j in 0..period {
                if s[j] != s[i * period + j] {
                    return false;
                }
            }
        }        
        return true;
    }
}
