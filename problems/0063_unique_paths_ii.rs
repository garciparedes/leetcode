impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (obstacle_grid.len(), obstacle_grid[0].len());
        
        let mut dp = vec![vec![0; m]; n];
        
        for i in 0..n {
            if  obstacle_grid[i][0] == 1 {
                break;
            }
            dp[i][0] = 1;
        }
        
        for j in 0..m {
            if obstacle_grid[0][j] == 1 {
                break;
            }
            dp[0][j] = 1;
        }
        
        for i in 1..n {
            for j in 1..m {
                if obstacle_grid[i][j] == 1 {
                    continue;
                }
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
        
        return dp[n - 1][m - 1];
    }
}
