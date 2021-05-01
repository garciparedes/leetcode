impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut ans = String::new();
        let s: Vec<_> = s.chars().collect();
        for chunk in s.chunks(2) {
            let character = chunk[0];
            ans.push(character);
            
            if chunk.len() < 2 {
                break;
            }
            
            let number = chunk[1];
            let decoded = (character as u8 + number as u8 - '0' as u8) as char;
            ans.push(decoded);
        }
        return ans;
    }
}
