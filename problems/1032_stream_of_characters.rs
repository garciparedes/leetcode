use std::collections::HashMap;

#[derive(Debug)]
struct TrieNode {
    is_word: bool,
    next: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode { 
            is_word: false, 
            next: HashMap::new(), 
        }
    }
    
    fn extend(&mut self, words: Vec<String>) {
        for word in words {
            self.push(word)
        }
    }
    
    fn push(&mut self, word: String) {
        let mut node = self;
        
        for c in word.chars().rev() {
            node = node.next.entry(c).or_insert_with(Self::new)
        }
        node.is_word = true;
    }
}

struct StreamChecker {
    root: TrieNode,
    letters: Vec<char>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamChecker {

    fn new(words: Vec<String>) -> Self {
        let mut root = TrieNode::new();
        root.extend(words);
        
        return StreamChecker {
            root: root,
            letters: Vec::new(),
        };
    }
    
    fn query(&mut self, letter: char) -> bool {
        self.letters.push(letter);
        let mut node = &self.root;
        
        for c in self.letters.iter().rev() {
            node = match node.next.get(&c) {
                Some(v) => v,
                None => break,
            };
            
            if node.is_word {
                return true;
            }
        }
         
        return false;
    }
}

/**
 * Your StreamChecker object will be instantiated and called as such:
 * let obj = StreamChecker::new(words);
 * let ret_1: bool = obj.query(letter);
 */
