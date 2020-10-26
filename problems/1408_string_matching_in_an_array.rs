impl Solution {
    pub fn string_matching(mut words: Vec<String>) -> Vec<String> {
        words.sort_unstable_by_key(|w| w.len());
        
        let mut ans = Vec::new();
        for i in 0..words.len() {
            for j in i + 1..words.len() {
                if words[j].contains(&words[i]) {
                    ans.push(words[i].clone());
                    break;
                }
            }
        }
        return ans;
    }
}
