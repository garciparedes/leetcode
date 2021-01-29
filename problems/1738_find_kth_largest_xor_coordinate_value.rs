use std::cmp;

impl Solution {
    pub fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let (n, m, k) = (matrix.len(), matrix[0].len(), k as usize);
        let mut xored = matrix.clone();

        for i in 0..n {
            for j in 1..m {
                xored[i][j] ^= xored[i][j - 1];
            }
        }
        
        for j in 0..m {
            for i in 1..n {
                xored[i][j] ^= xored[i - 1][j];
            }
        }
        
        let mut flatten = Vec::new();
        for i in 0..n {
            flatten.extend(&xored[i]);
        }
        flatten.sort_unstable_by_key(|&x| cmp::Reverse(x));
        return flatten[k - 1];
    }
}
