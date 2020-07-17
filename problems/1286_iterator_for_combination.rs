struct CombinationIterator {
    indexes: Vec<usize>,
    characters: Vec<char>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CombinationIterator {

    fn new(characters: String, combinationLength: i32) -> Self {
        Self {
            characters: characters.chars().collect(), 
            indexes: (0..combinationLength as usize).collect(),
        }
    }
    
    fn next(&mut self) -> String {
        let item = self.indexes
            .iter()
            .map(|&index| self.characters[index])
            .collect();
        self.increment();
        return item
    }
    
    fn increment(&mut self) {        
        let mut i = self.indexes.len() - 1;
        self.indexes[i] += 1;
        while i > 0 && !(self.indexes[i] < self.characters.len() - (self.indexes.len() - (i + 1))) {
            i -= 1;
            self.indexes[i] += 1;
            for j in i + 1..self.indexes.len() {
                self.indexes[j] = self.indexes[j - 1] + 1;   
            }
        }
    }
    
    fn has_next(&self) -> bool {
        return self.indexes
            .iter()
            .rev()
            .enumerate()
            .all(|(i, &index)| index + i < self.characters.len());
    }
    
    fn offset(&self) -> usize {
        self.characters.len() - self.indexes.len()
    }
}

/**
 * Your CombinationIterator object will be instantiated and called as such:
 * let obj = CombinationIterator::new(characters, combinationLength);
 * let ret_1: String = obj.next();
 * let ret_2: bool = obj.has_next();
 */

