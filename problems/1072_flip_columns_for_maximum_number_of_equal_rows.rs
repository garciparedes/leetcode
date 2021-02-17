use std::collections::HashMap;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let mut counter = HashMap::new();
        for row in matrix {
            let inverted = Self::invert(&row);
            *counter.entry(Self::serialize(&row)).or_insert(0) += 1;
            *counter.entry(Self::serialize(&inverted)).or_insert(0) += 1;
        }
        
        return *counter
            .values()
            .max()
            .unwrap_or(&0);
    }
    
    fn invert(row: &[i32]) -> Vec<i32> {
        let mut ans = Vec::new();
        for v in row {
            ans.push(1 - v);
        }
        return ans;
    }
    
    fn serialize(row: &[i32]) -> String {
        row
            .into_iter()
            .map(|v| if *v == 0 { 'a' } else { 'b' })
            .collect()
    }
} 

