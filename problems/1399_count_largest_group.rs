use std::collections::HashMap;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut counter = HashMap::new();
        for i in 1..n + 1 {
            *counter.entry(Self::sum_digits(i)).or_insert(0) += 1;
        }
        
        let mut counts: Vec<_> = counter.values().collect();
        counts.sort_unstable_by_key(|&x| -x);
        
        let mut i = 1;
        while i < counts.len() && counts[0] == counts[i] {
            i += 1;
        }
        return i as i32;
    }
    
    fn sum_digits(mut value: i32) -> i32 {
        let mut result = 0; 
        while value != 0 {
            result += value % 10;
            value /= 10;
        }
        return result;
    }
}
