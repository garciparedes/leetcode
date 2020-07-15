use std::collections::VecDeque;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let mut words = VecDeque::new();
        let mut i = 0;
        let mut current = String::new();
        for j in 0..s.len() {
            if s[j] != ' ' {
                current.push(s[j]);
            } else if current.len() > 0 {
                words.push_front(current.clone());
                current.clear();
            }
        }
        if current.len() > 0 {
            words.push_front(current.clone());
        }
        return Vec::from(words).join(" ");
    }
}
