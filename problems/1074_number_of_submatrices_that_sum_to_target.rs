use std::collections::HashMap;

impl Solution {
    pub fn num_submatrix_sum_target(mut matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let (n, m) = (matrix.len(), matrix[0].len());
        for i in 0..n {
            for j in 1..m {
                matrix[i][j] += matrix[i][j - 1];
            }
        }

        let mut ans = 0;
        let mut counter = HashMap::new();
        for i in 0..m {
            for j in i..m {
                counter.clear();
                counter.insert(0, 1);
                
                let mut current = 0;
                for k in 0..n {
                    current += matrix[k][j];
                    if i > 0 {
                        current -= matrix[k][i - 1];
                    }
                    ans += counter.get(&(current - target)).unwrap_or(&0);
                    
                    let tmp = counter.get(&current).unwrap_or(&0) + 1;
                    counter.insert(current, tmp);
                }
            }
        }
        
        return ans;
    }
}
