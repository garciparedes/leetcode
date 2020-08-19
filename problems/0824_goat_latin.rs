impl Solution {
    pub fn to_goat_latin(s: String) -> String {
        s.split(" ")
            .enumerate()
            .map(|(index, word)| {
                let mut characters: Vec<_> = word.chars().collect();    
                
                if !Self::is_vowel(characters[0]) {
                    let c = characters.remove(0);
                    characters.push(c);
                } 
                
                characters.push('m');
                for i in 0..index + 2 {
                    characters.push('a');
                }
                
                return characters
                    .into_iter()
                    .collect::<String>();
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
    
    fn is_vowel(c: char) -> bool {
        c == 'a' || c == 'A'
        || c == 'e' || c == 'E'
        || c == 'i' || c == 'I'
        || c == 'o' || c == 'O'
        || c == 'u' || c == 'U'
    }
}
