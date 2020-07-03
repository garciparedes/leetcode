impl Solution {
    pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (n, m) = (matrix.len(), matrix[0].len());
        
        let mut columns = Vec::new();
        for j in 0..m {
            let mut current = i32::min_value();
            for i in 0..n {
                if matrix[i][j] > current {
                    current = matrix[i][j]
                }
            }
            columns.push(current);
        }
        
        let mut lucky = Vec::new();
        for i in 0..n {
            let mut k = 0;
            for j in 0..m {
                if matrix[i][j] < matrix[i][k] {
                    k = j;
                }
            }
            if matrix[i][k] == columns[k] {
                lucky.push(columns[k]);
            }
        }
        return lucky;
    }
}
