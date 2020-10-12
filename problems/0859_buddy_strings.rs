use std::collections::HashSet;

impl Solution {
    pub fn buddy_strings(a: String, b: String) -> bool {
        let a: Vec<_> = a.chars().collect();
        let b: Vec<_> = b.chars().collect();
        
        if a.len() != b.len() {
            return false;
        }
        let mut occurrences = HashSet::new();
        let mut duplicates = false;
        let mut distinct = Vec::new();
        for (i, (ca, cb)) in a.iter().zip(b.iter()).enumerate() {
            if !duplicates {
                if !occurrences.insert(ca) {
                    duplicates = true;
                }
            }
            if ca == cb {
                continue;
            }
            distinct.push(i);
            if distinct.len() > 2 {
                return false;
            }
        }
        if distinct.len() == 0 {
            return duplicates;
        } else if distinct.len() == 2 {
            return a[distinct[0]] == b[distinct[1]] && a[distinct[1]] == b[distinct[0]];
        } else {
            return false;
        }
    }
}
