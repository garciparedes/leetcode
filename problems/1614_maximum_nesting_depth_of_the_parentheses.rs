use std::cmp;

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut ans = 0;
        let mut depth = 0;
        for c in s.chars() {
            if c != '(' && c != ')' {
                continue;
            }
            if c == '(' {
                depth += 1;
                ans = cmp::max(ans, depth);
            } else {
                depth -= 1;
            }
        }
        return ans;
    }
}
