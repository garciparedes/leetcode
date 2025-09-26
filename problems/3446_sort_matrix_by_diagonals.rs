impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let mut upper = Vec::new();
        let mut lower = Vec::new();
        for i in 0..n {
            upper.push(grid[i][i]);
        }
        upper.sort();
        for i in 0..n {
            grid[i][i] = upper.pop().unwrap()
        }
        for i in 1..n {
            for j in 0..(n - i) {
                upper.push(grid[j][i + j]);
                lower.push(grid[i + j][j]);
            }
            upper.sort_by_key(|x| -x);
            lower.sort();
            for j in 0..(n - i) {
                grid[j][i + j] = upper.pop().unwrap();
                grid[i + j][j] = lower.pop().unwrap();
            }
        }
        grid
    }
}
