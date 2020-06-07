use std::collections::HashSet;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut result = String::new();     
        let mut counter = 0;
        let mut precondition = false;
        for character in s.chars() {
            precondition = (counter != 0);
            if character == '(' {
                counter += 1;
            } else {
                counter -= 1;
            }
            if precondition && counter != 0 {
                result.push(character);
            }
            
        }
        return result;
    }
}
