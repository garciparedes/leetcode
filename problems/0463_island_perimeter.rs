impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());
        
        let mut perimeter = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 0 {
                    continue;
                } 
                if i == 0 || grid[i - 1][j] == 0 {
                    perimeter += 1;
                }
                if i == n - 1 || grid[i + 1][j] == 0 {
                    perimeter += 1;
                }
                if j == 0 || grid[i][j - 1] == 0 {
                    perimeter += 1;
                }
                if j == m - 1 || grid[i][j + 1] == 0 {
                    perimeter += 1;
                }
            }
        }
        return perimeter;
    }
}
