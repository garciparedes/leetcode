use std::collections::VecDeque;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut characters: VecDeque<_> = s.to_lowercase().to_string().chars().collect();
        
        while !characters.is_empty() {
            while let Some(head) = characters.pop_front() {
                if !head.is_ascii_alphanumeric() {
                    continue;
                }
                while let Some(tail) = characters.pop_back() {
                    if !tail.is_ascii_alphanumeric() {
                        continue;
                    }
                    if head != tail {
                        return false;
                    }
                    break;
                }
                break;
            }
        }
        return true;
    }
}
