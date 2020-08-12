use std::cmp;

impl Solution {
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len() as i32, grid[0].len() as i32);
        
        let mut best = 0;
        for i in 0..n as i32 {
            for j in 0..m as i32 {
                best = cmp::max(best, Self::mine(grid.clone(), i, j));
            }
        }
        return best;
    }
    
    fn mine(mut grid: Vec<Vec<i32>>, x: i32, y: i32) -> i32 {
        let (n, m) = (grid.len() as i32, grid[0].len() as i32);
        if x < 0 || y < 0 || x == n || y == m || grid[x as usize][y as usize] == 0 {
            return 0;
        }        
        let current = grid[x as usize][y as usize];
        grid[x as usize][y as usize] = 0;
        
        let mut best = 0;
        for (i, j) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            best = cmp::max(best, Self::mine(grid.clone(), x + i, y + j));
        }
        best += current;
        return best;
    }
}
