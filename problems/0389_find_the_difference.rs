impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut set = vec![0; 26];
        
        for c in t.chars() {
            set[(c as u8 - 'a' as u8) as usize] += 1;
        }
        
        for c in s.chars() {
            set[(c as u8 - 'a' as u8) as usize] -= 1;
        }
        
        let mut i = 0;
        while i < set.len() && !(set[i] > 0) {
            i += 1;
        }
        
        return (i as u8 + 'a' as u8) as char;
    }
}
