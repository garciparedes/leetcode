use std::collections::{
    HashSet,
    HashMap,
};

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut memory = HashMap::new();
        let word_dict: HashSet<_> = word_dict.iter().map(|word| &word[..]).collect();
        
        let mut result = Vec::new();
        for j in 1..s.len() + 1 {
            result.extend(Self::rec(&s[..], 0, j, &word_dict, &mut memory).into_iter());
        } 
        return result;
    }
    
    fn rec(
        s: &str, 
        i: usize, 
        j: usize, 
        word_dict: &HashSet<&str>, 
        memory: &mut HashMap<(usize, usize), Vec<String>>,
    ) -> Vec<String> {
        if let Some(result) = memory.get(&(i, j)) {
            return result.clone();
        }
        
        let mut result = Vec::new();
        if word_dict.contains(&s[i..j]) {
            if j == s.len() {
                result.push(String::from(&s[i..j]));
            } else {
                for k in j..s.len() + 1 {
                       let iterable = Self::rec(s, j, k, word_dict, memory)
                        .into_iter()
                        .map(|sentence| format!("{} {}", &s[i..j], sentence));
                    result.extend(iterable);   
                }    
            }
        }

        memory.insert((i, j), result.clone());
        return result;
    }
}
