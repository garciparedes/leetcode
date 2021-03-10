impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut it1 = word1.chars().peekable();
        let mut it2 = word2.chars().peekable();
        
        let mut ans = String::new();
        while it1.peek().is_some() || it2.peek().is_some() {
            if let Some(c) = it1.next() {
                ans.push(c);
            }
            if let Some(c) = it2.next() {
                ans.push(c);
            }
        }
        return ans;
    }
}
