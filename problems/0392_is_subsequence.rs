impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut iterator = s.chars();
        let mut  current = match iterator.next() {
            Some(x) => x,
            None => return true,
        };
        for c in t.chars() {
            if c != current {
                continue;
            }
            current = match iterator.next() {
                Some(x) => x,
                None => return true,
            };
        }
        return false;
    }
}
