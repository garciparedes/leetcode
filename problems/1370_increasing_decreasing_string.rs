impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut result: Vec<char> = Vec::new();
        let mut characters: Vec<char> = s.chars().collect();
        characters.sort();
        
        while !characters.is_empty() {            
            result.push(characters.remove(0));
            let mut i = 0;
            while i < characters.len() {
                if characters[i] <= result[result.len() - 1] {
                    i += 1;
                    continue;
                }
                result.push(characters.remove(i));
            }
            
            
            if let Some(c) = characters.pop() {
                result.push(c);
            } else {
                break;
            }

            let mut i = characters.len();
            while 0 < i {
                i -= 1;
                if characters[i] >= result[result.len() - 1] {
                    continue;
                }
                result.push(characters.remove(i));

            }
        }
        return result.iter().collect();
    }
}
