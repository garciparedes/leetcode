use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut counter = HashMap::new();
        for c in s.chars() {
            *counter.entry(c).or_insert(0) += 1;
        }
        let mut odd = false;
        let mut result = 0;
        for count in counter.values() {
            if (count % 2 == 0) {
                result += count; 
            } else if !odd {
                result += count; 
                odd = true;
            } else {
                result += count - 1;
            }
        }
        return result;
    }
}
