use std::collections::{
    HashMap,
    HashSet,
};

use std::cmp;

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        
        let s_counter = Solution::frequencies(&s);
        let t_counter = Solution::frequencies(&t);
        
        let keys: HashSet<&char> = s_counter
            .keys()
            .chain(t_counter.keys())
            .collect();
        
        return keys
            .iter()
            .map(|&k| {
                let diff = (
                    s_counter.get(k).unwrap_or(&0) 
                    - t_counter.get(k).unwrap_or(&0)
                );
                
                return vec![cmp::max(0, diff), cmp::max(0, -diff)];
            })
            .fold(vec![0, 0], |acc, x| vec![acc[0] + x[0], acc[1] + x[1]])
            .into_iter()
            .min()
            .unwrap();
    }
    
    fn frequencies(s: &str) -> HashMap<char, i32> {
        let mut counter = HashMap::new();
        for c in s.chars() {
            *counter.entry(c).or_insert(0) += 1;
        }
        return counter;
    }
}
