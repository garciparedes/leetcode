use std::collections::{
    HashMap,
    BTreeMap,
};

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut memory: HashMap<BTreeMap<char, i32>, Vec<String>> = HashMap::new();
        for word in strs {
            let mut current = BTreeMap::new();
            for c in word.chars() {
                *current.entry(c).or_insert(0) += 1;
            }
            memory.entry(current).or_insert(Vec::new()).push(word);
        }
        return memory.values().cloned().collect();
    }
}
