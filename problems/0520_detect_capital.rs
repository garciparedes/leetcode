impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut iterable = word.chars();
        let initial = iterable.next().unwrap().is_ascii_uppercase();
        let all = iterable.next().unwrap_or('a').is_ascii_uppercase();
        if !initial && all {
            return false;
        }
        for character in iterable {
            if all != character.is_ascii_uppercase()  {
                return false;
            }
        }
        return true;
    }
}
