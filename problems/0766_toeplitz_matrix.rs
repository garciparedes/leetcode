impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let (n, m) = (matrix.len(), matrix[0].len());
        
        for j in 0..m  {
            let v = matrix[0][j];
            for i in 1..n {
                if i + j == m {
                    break;
                }
                if v != matrix[i][i + j] {
                    return false;
                }
            }   
        }
        for i in 1..n {
            let v = matrix[i][0];
            for j in 1..m {
                if i + j == n {
                    break;
                }
                if v != matrix[i + j][j] {
                    return false;
                }
            }
        }
        return true;
    }
}
