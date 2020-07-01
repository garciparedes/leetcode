use std::cmp;

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (matrix.len(), matrix[0].len());
        let mut count = 0;
        for i in 0..n {
            for j in 0..m {
                count += Self::count_increasing(&matrix, i, j);
            }
        }
        return count as i32;
    }
    
    fn count_increasing(matrix: &Vec<Vec<i32>>, i: usize, j: usize) -> usize {
        let ij = cmp::min(i, j) + 1;
        for k in 0..ij {
            if !Self::count_one(&matrix, i, j, k) {
                return k;
            }
        }
        return ij;
    }
    
    fn count_one(matrix: &Vec<Vec<i32>>, i: usize, j: usize, k: usize) -> bool {
        for kk in 0..k {
            if !(matrix[i - kk][j - k] == 1 && matrix[i - k][j - kk] == 1) {
                return false;
            }
        }
        return matrix[i - k][j - k] == 1;
    }
}
