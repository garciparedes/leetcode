impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let (n, m) = (grid.len(), grid[0].len());        
        let mut ans = vec![vec![0; m]; n];
        
        for a in 0..n * m {
            let b = (a + k) % (n * m);
            
            let (i, j) = (a / m, a % m);
            let (x, y) = (b / m, b % m);
            
            ans[x][y] = grid[i][j];
        }
        return ans;
        
    }
}
