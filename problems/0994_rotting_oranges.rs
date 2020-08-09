impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());

        let mut iterations = 0;
        let mut prev_count = i32::max_value();
        let mut count = Self::count_fresh(&grid);;
        while count > 0 && prev_count > count {
            iterations += 1;
            prev_count = count;
            let tmp_grid = grid.clone();
            for i in 0..n {
                for j in 0..m {
                    if tmp_grid[i][j] == 2 {
                        if i > 0 && tmp_grid[i - 1][j] == 1 {
                            grid[i - 1][j] = 2
                        }
                        if i < n - 1 && tmp_grid[i + 1][j] == 1 {
                            grid[i + 1][j] = 2
                        }
                        if j > 0 && tmp_grid[i][j - 1] == 1 {
                            grid[i][j - 1] = 2
                        }
                        if j < m - 1 && tmp_grid[i][j + 1] == 1 {
                            grid[i][j + 1] = 2
                        }
                    }
                }
            }
            count = Self::count_fresh(&grid);
        }
        if count > 0 {
            return -1;
        }
        return iterations;
    }
    
    fn count_fresh(grid: &Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());
        let mut count = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    count += 1;
                }
            }
        }
        return count;
    }
}
