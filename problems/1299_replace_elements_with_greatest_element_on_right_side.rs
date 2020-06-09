use std::cmp;

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let n = arr.len();
        let mut result = vec![0; n];
        result[n - 1] = -1;
        
        let mut maximum = -1;
        for i in (0..n-1).rev() { 
            maximum = cmp::max(arr[i + 1], maximum);
            result[i] = maximum;
        }

        return result;
    }
}
