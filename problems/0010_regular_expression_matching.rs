impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if p.is_empty() {
            return s.is_empty();
        }
        
        let first_matched = !s.is_empty() && vec![s.chars().nth(0).unwrap(), '.'].contains(&p.chars().nth(0).unwrap());
        if p.len() >= 2 && p.chars().nth(1).unwrap() == '*' {
            return 
                (first_matched && Solution::is_match(s[1..].to_string(), p.clone())) 
                || Solution::is_match(s.to_string(), p[2..].to_string())
            ;
        } else {
            return first_matched && Solution::is_match(s[1..].to_string(), p[1..].to_string());
        }
    }
}
