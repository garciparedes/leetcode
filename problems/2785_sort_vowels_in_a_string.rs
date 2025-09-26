use std::cmp::Reverse;

const VOWELS: [char; 10] = ['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'];

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut v = Vec::new();
        for c in s.chars() {
            if VOWELS.contains(&c) {
                v.push(c)
            }
        }
        v.sort_by_key(|c| Reverse(*c));
        s.chars()
            .enumerate()
            .map(|(i, c)| if VOWELS.contains(&c) { v.pop().unwrap() } else { c })
            .collect()
    }
}
