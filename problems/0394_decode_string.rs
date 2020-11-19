impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut chars: Vec<_> = s.chars().collect();
        
        let mut ans = String::new();
        
        let mut capturing = false;
        let mut repetitions = 0;
        let mut encoded = String::new();
        let mut i = 0;
        while let Some(c) = chars.get(i) {
            if c.is_numeric() {
                ans.push_str(&Self::rec(&chars, &mut i));
            } else {
                ans.push(*c);
            }
            i += 1;
        }
        return ans;
    }
    
    fn rec(chars: &[char], i: &mut usize) -> String {
        let mut capturing = false;
        let mut repetitions: usize = 0;
        let mut encoded = String::new();
        while let Some(c) = chars.get(*i) {
            if let Some(d) = c.to_digit(10) {
                if capturing {
                    encoded.push_str(&Self::rec(chars, i));
                } else {
                    repetitions = 10 * repetitions + d as usize; 
                }
            } else if *c == '[' {
                capturing = true;
            } else if *c == ']' {
                return encoded.repeat(repetitions);
            } else if capturing {
                encoded.push(*c);
            }
            *i += 1;
        } 
        return String::new();
    }
}
