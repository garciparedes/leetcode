use std::cmp;

impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![0; col_sum.len()]; row_sum.len()];
        for i in 0..row_sum.len() {
            for j in 0..col_sum.len() {
                let tmp = cmp::min(row_sum[i], col_sum[j]);
                row_sum[i] -= tmp;
                col_sum[j] -= tmp;
                ans[i][j] = tmp;
            }
        }
        return ans;
    }
}
