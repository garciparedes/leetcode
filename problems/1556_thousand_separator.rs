impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        let mut characters: Vec<_> = n.to_string().chars().rev().collect();
        
        let mut count = 0;
        let mut i = 0;
        while i < characters.len() {
            if count == 3 {
                characters.insert(i, '.');
                count = -1;
            }
            count += 1;
            i += 1;
        }
        return characters.into_iter().rev().collect();
    }
}
