use std::cmp;

impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort();
        
        let mut total: i32 = satisfaction
            .iter()
            .sum();
        
        let mut i = 0;
        while i < satisfaction.len() && 0 >= total { 
            total -= satisfaction[i];
            i += 1;
        }
        
        return satisfaction[i..]
            .into_iter()
            .zip(1..satisfaction.len() + 1)
            .map(|(a, b)| a * b as i32)
            .sum();
    }
}
