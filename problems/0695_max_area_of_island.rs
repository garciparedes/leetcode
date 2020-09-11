use std::cmp;

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len() as isize, grid[0].len() as isize);
        
        let mut color = 2;
        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                let value = Self::dfs(&mut grid, i, j, color);
                if value > 0 {
                    ans = cmp::max(ans, value);
                    color += 1;
                }
            }
        }
        return ans;
    }
    
    fn dfs(grid: &mut Vec<Vec<i32>>, i: isize, j: isize, color: i32) -> i32 {
        if (
            i < 0 
            || j < 0 
            || i >= grid.len() as isize 
            || j >= grid[0].len() as isize 
            || grid[i as usize][j as usize] != 1
        ) {
            return 0;
        }
        grid[i as usize][j as usize] = color;
        let mut ans = 1;
        ans += Self::dfs(grid, i + 1, j, color);
        ans += Self::dfs(grid, i - 1, j, color);
        ans += Self::dfs(grid, i, j + 1, color);
        ans += Self::dfs(grid, i, j - 1, color);
        return ans;
    }
}
