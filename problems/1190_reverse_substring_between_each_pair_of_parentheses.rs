use std::collections::HashMap;

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let s: Vec<_> = s.chars().collect();
        let mut pairs = HashMap::new();
        let mut stack = Vec::new();
        for (i, v) in s.iter().enumerate() {
            if *v == '(' {
                stack.push(i);
            } else if *v == ')' {
                let j = stack.pop().unwrap();
                pairs.insert(i, j);
                pairs.insert(j, i);
            }
        }
        
        let mut ans = String::new();
        let (mut i, mut d) = (0, 1);
        while i < s.len() {
            if s[i] == '(' || s[i] == ')' {
                i = pairs[&i];
                d *= - 1;
            } else {
                ans.push(s[i]);
            }
            i = (i as isize + d) as usize;
        }
        
        return ans;
    }
}
