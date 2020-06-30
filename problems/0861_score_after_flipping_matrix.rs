use std::cmp;

impl Solution {
    pub fn matrix_score(mut a: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (a.len(), a[0].len());
        
        let mut best = 0;
        let mut improvement = n + m;
        while 0 < improvement {
            improvement -= 1;
            
            let mut matrix = a.clone();
            
            if improvement < n {
                let i = improvement;
                for j in 0..m {
                    matrix[i][j] = 1 - matrix[i][j];
                }
            } else {
                let j = improvement - n;
                for i in 0..n {
                    matrix[i][j] = 1 - matrix[i][j];
                }
            }    
            
            let current = Self::score(&matrix);
            if best >= current {
                continue;
            }
            a = matrix;
            best = cmp::max(best, current);
            improvement = n + m;
        }
        return best;
    }
    
    fn score(matrix: &Vec<Vec<i32>>) -> i32 {
        let mut score: i32 = 0;
        for row in matrix {
            score += row
                .iter()
                .rev()
                .enumerate()
                .map(|(j, &cell)| if cell == 1 { i32::pow(2, j as u32) } else { 0 })
                .sum::<i32>();
        }
        return score;
    }
}
