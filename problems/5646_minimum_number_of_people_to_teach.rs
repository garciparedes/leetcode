use std::collections::HashSet;

impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        let mut best = i32::max_value();
        let languages = languages.into_iter().map(|entry| entry.into_iter().collect::<HashSet<_>>()).collect::<Vec<_>>();
        
        let mut skipable = vec![false; friendships.len()];
        
        for (i, pair) in friendships.iter().enumerate() { 
            for a in &languages[pair[0] as usize - 1] {
                if languages[pair[1] as usize - 1].contains(&a) {
                    skipable[i] = true;
                    break;
                }
            } 
        
        }
        
        
        for i in 1..=n {
            let mut current = 0;
            let mut tmp = languages.clone();
            
            for (j, pair) in friendships.iter().enumerate() { 
                if skipable[j] {
                    continue;
                }
                
                if tmp[pair[0] as usize - 1].insert(i) {
                    current += 1;
                }
                if tmp[pair[1] as usize - 1].insert(i) {
                    current += 1;
                }
            }
            if current < best {
                best = current;
            }
        }
        return best;
        
    }
}
