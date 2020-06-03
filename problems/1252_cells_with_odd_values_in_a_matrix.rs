impl Solution {
    pub fn odd_cells(n: i32, m: i32, indices: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (n as usize, m as usize);
        let mut matrix: Vec<Vec<i32>> = vec![vec![0; m]; n];
        
        for indice in indices {
            let r_i = indice[0] as usize;
            for j in 0..m {
                matrix[r_i][j] += 1;    
            }

            let c_i = indice[1] as usize;
            for j in 0..n {
                matrix[j][c_i] += 1
            }
        }
            
        let result = matrix.into_iter().fold(0, |cum, row| {
            cum + row.into_iter().filter(|cell| cell % 2 == 1).count() as i32
        });

        return result;
    }
}
