use std::collections::HashMap;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let n = s.len();
        if n < 11 {
            return Vec::new();
        }
        let mut counter = HashMap::new();
        for i in 0..(n + 1 - 10) {
            *counter.entry(&s[i..(i + 10)]).or_insert(0) += 1;
        }   
        return counter
            .into_iter()
            .filter(|(_, v)| *v > 1)
            .map(|(k, _)| k.to_string())
            .collect();
    }
}
