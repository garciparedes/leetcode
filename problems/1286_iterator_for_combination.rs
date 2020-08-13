use std::ops::Range;

struct CombinationIterator {
    characters: Vec<char>,
    indexes: Vec<usize>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CombinationIterator {

    fn new(characters: String, combinationLength: i32) -> Self {
        CombinationIterator { 
            characters: characters.chars().collect(), 
            indexes: (0..combinationLength as usize).collect(),
        }
    }
    
    fn next(&mut self) -> String {
        let result = self
            .indexes
            .iter()
            .map(|&index| self.characters[index])
            .collect();
        
        self.increment();
        return result;
    }
    
    fn increment(&mut self) {
        let mut i = self.indexes.len() - 1;
        while i > 0 && self.indexes[i] == self.characters.len() - (self.indexes.len() - i) {
            i -= 1;
        }
        
        self.indexes[i] += 1;
        for j in i + 1 .. self.indexes.len() {
            self.indexes[j] = self.indexes[j - 1] + 1;
        } 
    }
    
    fn has_next(&self) -> bool {
        self.indexes
            .iter()
            .zip(self.threshold())
            .all(|(&a, b)| a <= b)
    }
    
    fn threshold(&self) -> Range<usize> {
        (self.characters.len() - self.indexes.len())..self.characters.len()
    }
}

/**
 * Your CombinationIterator object will be instantiated and called as such:
 * let obj = CombinationIterator::new(characters, combinationLength);
 * let ret_1: String = obj.next();
 * let ret_2: bool = obj.has_next();
 */
