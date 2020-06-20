use std::collections::HashMap;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut ranges: HashMap<char, Vec<i32>> = HashMap::new();
        
        
        for (i, c) in s.chars().enumerate() {
            let mut range = ranges.entry(c).or_insert_with(Vec::new);
            if range.len() < 2 {
                range.push(i as i32);    
            } else {
                range[1] = i as i32;
            }   
        }
        
        let mut ranges: Vec<Vec<i32>> = ranges
            .values()
            .cloned()
            .collect();
        ranges.sort();

        let mut result = Vec::new();
        let mut low = ranges[0][0];
        let mut upper = ranges[0][ranges[0].len() - 1];
        for range in ranges {
            if range.iter().all(|&v| v > upper)  {
                result.push(upper - low + 1);
                low = range[0];
                upper = range[range.len() - 1];
            } else if upper < range[range.len() - 1] {
                upper = range[range.len() - 1];
            }
        }
        result.push(upper - low + 1);
        return result;
    }
}
