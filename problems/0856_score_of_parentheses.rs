impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        Self::helper(&s)
    }
    
    fn helper(s: &str) -> i32 {
        if s.len() == 0 {
            return 1;
        }
        let mut ans = 0;
        let mut begin = 0;
        let mut depth = 0;
        for (end, c) in s.chars().enumerate() {
            if c == '(' {
                if depth == 0 {
                    begin = end + 1;
                }
                depth += 1;
            } else {
                depth -= 1;
                if depth == 0 {
                    if begin == end {
                        ans += 1;
                    } else {
                        ans += 2 * Self::helper(&s[begin..end]);
                    }
                }
            }
        }
        return ans;
    }
}
