impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        
        let mut matrix = vec![vec![-1; n]; n];        
        let mut current = 1;
        
        for i in 0..(n / 2)  {
            let base_len = n - 2 * i;
            
            for k in 0..base_len {
                matrix[i][i + k] = current;        
                current += 1;
            }
            
            for k in (0..base_len - 2) {
                matrix[(i + 1) + k][n - (i + 1)] = current;        
                current += 1;
            }
            
            for k in (0..base_len).rev() {
                matrix[n - (i + 1)][i + k] = current;        
                current += 1;
            }
            
            for k in (0..base_len - 2).rev() {
                matrix[(i + 1) + k][i] = current;        
                current += 1;
            }
        }
        if n % 2 == 1 {
            matrix[n / 2][n / 2] = current;
        }
        
        return matrix;
        
        
    }
}
