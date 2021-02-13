impl Solution {
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let color = (2 + i * grid[0].len() + j) as i32;
                Self::explore(&mut grid, i, j, color);
            }
        }
        let mut ans = 0;
        for i in 1..grid.len() - 1 {
            for j in 1..grid[0].len() - 1 {
                if grid[i][j] < 2 {
                    continue;
                }
                let color = (2 + i * grid[0].len() + j) as i32;
                if Self::evaluate(&mut grid, i, j, color) {
                    ans += 1;
                }
            }
        }
        return ans;
        
    }
    
    fn explore(grid: &mut Vec<Vec<i32>>, i: usize, j: usize, color: i32) {
        if grid[i][j] != 0 {
            return;
        }
        
        grid[i][j] = color;
        
        for (it, jt) in vec![(1, 0), (-1, 0), (0, 1), (0, -1)].into_iter() {
            let ii = (i as isize + it);
            let jj = (j as isize + jt);
            
            if ii < 0 || grid.len() as isize <= ii || jj < 0 || grid[0].len() as isize <= jj {
                continue;
            } 
            
            Self::explore(grid, ii as usize, jj as usize, color);
        }
    }
    
    fn evaluate(grid: &mut Vec<Vec<i32>>, i: usize, j: usize, color: i32) -> bool {
        if grid[i][j] < 2 {
            return true
        }
        if i == 0 || i == grid.len() - 1 || j == 0 || j == grid[0].len() - 1 || grid[i][j] != color {
            return false;
        }
        grid[i][j] = 0;
        
        for (it, jt) in vec![(1, 0), (-1, 0), (0, 1), (0, -1)].into_iter() {
            let ii = (i as isize + it);
            let jj = (j as isize + jt);
            if !Self::evaluate(grid, ii as usize, jj as usize, color) {
                return false;
            }
        }
        return true;
    }
    
    
}
