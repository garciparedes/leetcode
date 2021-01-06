impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        if Self::is_palindrome(&s) {
            return 1;
        } 
        
        return 2;
    }
    
    fn is_palindrome(s: &str) -> bool {
        s == s.chars().rev().collect::<String>()
    }
}
