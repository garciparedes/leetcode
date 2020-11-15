impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        
        let mut left_b = 0;        
        let mut right_a = 0;
        for i in 0..n {
            if s[i] == 'a' {
                right_a += 1;
            }
        }
        
        let mut best = right_a;
        for i in 0..n {
            if s[i] == 'a' {
                right_a -= 1;
            } else {
                left_b += 1;
            }
            let tmp = left_b + right_a;
            if tmp < best {
                best = tmp;
            }
        }
        return best;
        
    }
}
