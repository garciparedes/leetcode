impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let half_n = s.len() / 2;
        
        return s[..half_n].chars().filter(|&c| Self::is_vowel(c)).count() == s[half_n..].chars().filter(|&c| Self::is_vowel(c)).count()
    }
    
    fn is_vowel(c: char) -> bool {
        c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
            || c == 'A' || c == 'E' || c == 'I' || c == 'O' || c == 'U'
    }
}

