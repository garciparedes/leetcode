use std::collections::{
    HashMap,
    HashSet,
};

impl Solution {
    pub fn num_special_equiv_groups(a: Vec<String>) -> i32 {
        let mut memory = HashSet::new();
        for word in a {
            
            let (mut even, mut odd) = (HashMap::new(), HashMap::new());
            for (i, character) in word.chars().enumerate() {
                if i % 2 == 0 {
                    *even.entry(character).or_insert(0) += 1;
                } else {
                    *odd.entry(character).or_insert(0) += 1;
                }
            }
            
            let mut even: Vec<(char, usize)> = even.into_iter().collect();
            even.sort();
            
            let mut odd: Vec<(char, usize)> = odd.into_iter().collect();
            odd.sort();
            
            memory.insert((even, odd));
        }
        return memory.len() as i32;
    }
}
