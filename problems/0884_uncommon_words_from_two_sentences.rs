use std::collections::HashMap;

impl Solution {
    pub fn uncommon_from_sentences(a: String, b: String) -> Vec<String> {
        let mut counter = HashMap::new();
        for word in a.split(" ") {
            *counter.entry(word).or_insert(0) += 1;
        }
        for word in b.split(" ") {
            *counter.entry(word).or_insert(0) += 1;
        }
        return counter
            .into_iter()
            .filter(|(_, count)| count == &1)
            .map(|(word, _)| word.to_string())
            .collect();
    }
}
