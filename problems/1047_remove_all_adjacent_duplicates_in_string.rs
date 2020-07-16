use std::collections::VecDeque;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut characters: VecDeque<char> = s.chars().collect();        
        let mut i = 1;
        while i < characters.len() {
            if characters[i] == characters[i - 1] {
                characters.remove(i - 1);
                characters.remove(i - 1);
                if i > 1 {
                    i -= 1;
                }
            } else {
                i += 1;
            }
        }
        return characters.into_iter().collect();
    }
}
