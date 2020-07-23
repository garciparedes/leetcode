use std::cmp::Ordering;

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort_unstable();
        
        let mut result = vec![vec![arr[0], arr[1]]];
        for i in 2..arr.len() {
            match (arr[i] - arr[i - 1]).cmp(&(result[0][1] - result[0][0])) {
                Ordering::Less => {
                    result.clear();
                    result.push(vec![arr[i - 1], arr[i]]);
                }
                Ordering::Equal => {
                    result.push(vec![arr[i - 1], arr[i]]);
                }
                _ => (),
            }
        }
        
        return result;
    }
}
