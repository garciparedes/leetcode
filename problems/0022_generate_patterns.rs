impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans = Vec::new();
        Self::backtrack(&mut ans, String::new(), 0, 0, n as usize);
        return ans;
    }
    
    fn backtrack(ans: &mut Vec<String>, current: String, open: usize, close: usize, n: usize) {
        if current.len() == 2 * n {
            ans.push(current);
            return;
        }
        
        if open < n {
            Self::backtrack(ans, format!("{}(", current), open + 1, close, n);            
        } 
        
        if close < open {
            Self::backtrack(ans, format!("{})", current), open, close + 1, n);
        }
    }
}
