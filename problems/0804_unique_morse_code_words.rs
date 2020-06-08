use std::collections::HashSet;

static MORSE: [&str; 26] = [
    ".-","-...","-.-.","-..",".","..-.","--.","....","..",".---",
    "-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-",
    "..-","...-",".--","-..-","-.--","--.."
];


impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        words
            .iter()
            .map(|word| word.chars().map(|c| MORSE[c as usize - 97]).collect::<String>())
            .collect::<HashSet<String>>()
            .len() as i32
    }
}
