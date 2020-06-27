use std::cmp;

impl Solution {
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let (n, m, k) = (mat.len() as isize, mat[0].len() as isize, k as isize);
        let mut result  = vec![vec![0; m as usize]; n as usize];
        for i in 0..n {
            for j in 0..m {
                for r in cmp::max(0, i - k)..cmp::min(n, i + k + 1) {
                    for c in cmp::max(0, j - k)..cmp::min(m, j + k + 1) {
                        result[i as usize][j as usize] += mat[r as usize][c as usize];
                    }
                }
            }
        }
        return result;
    }
}
