use std::cmp;
use std::collections::HashMap;

impl Solution {
    pub fn interval_intersection(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        
        let (mut i_a, mut i_b) = (0, 0);
        while (i_a < a.len()) && (i_b < b.len()) {
            let item = vec![cmp::max(b[i_b][0], a[i_a][0]), cmp::min(a[i_a][1], b[i_b][1])];
            if item[0] <= item[1] {
                result.push(item);
            }
            if b[i_b][1] < a[i_a][1] {
                i_b += 1;
            } else {
                i_a += 1;
            }
        }
        
        return result;
    }
}
