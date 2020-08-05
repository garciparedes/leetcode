use std::collections::HashMap;

struct WordDictionary {
    leaf: bool,
    characters: HashMap<char, WordDictionary>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    /** Initialize your data structure here. */
    fn new() -> Self {
        WordDictionary { leaf: false, characters: HashMap::new(), }
    }
    
    /** Adds a word into the data structure. */
    fn add_word(&mut self, word: String) {
        if let Some(character) = word.chars().next() {
            self.characters
                .entry(character)
                .or_insert_with(WordDictionary::new)
                .add_word(word[1..].to_string());
        } else {
            self.leaf = true;
        }
    }
    
    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    fn search(&self, word: String) -> bool {
        match word.chars().next() {
            Some(character) => {
                match character {
                    '.' => {
                        self.characters
                            .values()
                            .any(|other| other.search(word[1..].to_string()))
                    },
                    _ => {
                      match self.characters.get(&character) {
                          Some(other) => other.search(word[1..].to_string()),
                          None => false,
                        }
                    }
                }
            },
            None => self.leaf,
        }
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */
