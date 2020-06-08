static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];

impl Solution {
    pub fn freq_alphabets(s: String) -> String {        
        let mut result = String::new();
        let characters: Vec<char> = s.chars().collect();
        let mut current = String::new();
        let mut i = 0;
        while i < characters.len() {
            current.clear();
            current.push(characters[i]);
            
            if i + 2 < characters.len() && characters[i+2] == '#' {
                current.push(characters[i + 1]);
                i += 2;
            }
            
            let index = current.parse::<usize>().unwrap();
            result.push(ASCII_LOWER[index - 1]);
            
            i += 1;
            
        }
        return result;
    }
}
