impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut seen = vec![false; 26];
        
        for c in sentence.chars() {
            seen[(c as u8 - 'a' as u8) as usize] = true;
        }
        
        for s in seen {
            if !s {
                return false;
            }
        }
        
        return true;
    }
}
