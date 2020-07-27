impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let mut grid = Self::augment(grid);
        
        let mut count = 0;
        while let Some(origin) = Self::next_origin(&grid)  {
            count += 1;
            Self::dfs(&mut grid, origin.0, origin.1, count);
        }

        return count;
    }
    
    fn augment(grid: Vec<String>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let mut augmented = vec![vec![0; n * 3];n * 3];
        for (i, row) in grid.into_iter().enumerate() {
            for (j, cell) in row.chars().enumerate() {
                let (ii, jj) = (i * 3, j * 3);
                
                match cell {
                    '\\' => {
                        for k in 0..3 {
                            augmented[ii + k][jj + k] = -1;
                        }
                    },
                    '/' => {
                        for k in 0..3 {
                            augmented[ii + k][jj + 2 - k] = -1;
                        }
                    },
                    _ => (),
                }
            }    
        }
        return augmented;
    }   
    
    fn dfs(grid: &mut Vec<Vec<i32>>, x: usize, y: usize, color: i32) {
        if grid[x][y] != 0 {
            return;
        }
        
        grid[x][y] = color;
        
        if x > 0 {
            Self::dfs(grid, x - 1, y, color);
        }
        if x < grid.len() - 1 {
            Self::dfs(grid, x + 1, y, color);
        }
        if y > 0 {
            Self::dfs(grid, x, y - 1, color);
        }
        if y < grid[0].len() - 1 {
            Self::dfs(grid, x, y + 1, color);
        }
    }
    
    fn next_origin(grid: &Vec<Vec<i32>>) -> Option<(usize, usize)> {
        for (i, row) in grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == 0 {
                    return Some((i, j));
                }
            }
        }
        return None;
    }
}
