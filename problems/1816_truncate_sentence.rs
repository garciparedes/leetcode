impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let mut words: Vec<_> = s
        .trim()
        .split_whitespace()
        .collect();
        
        words.truncate(k as usize);
        
        return words.join(" ");
    }
}
