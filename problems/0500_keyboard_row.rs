const UPPER: &'static [char] = &['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'];
const MIDDLE: &'static [char] = &['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'];
const LOWER: &'static [char] = &['z', 'x', 'c', 'v', 'b', 'n', 'm'];


impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        words
            .into_iter()
            .filter(|word| {
                let word = word.to_lowercase();
                return (
                    word.chars().all(|c| UPPER.contains(&c)) 
                    || word.chars().all(|c| MIDDLE.contains(&c)) 
                    || word.chars().all(|c| LOWER.contains(&c)) 
                );
            })
            .collect()
    }
}
