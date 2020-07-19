impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let horizontal: i32 = grid
            .iter()
            .map(|row| row.iter().max().unwrap())
            .sum();
        
        let vertical: i32 = (0..grid[0].len())
            .map(|j| grid.iter().map(|row| row[j]).max().unwrap())
            .sum();
        
        let top: i32 = grid
            .iter()
            .map(|row| row.iter().filter(|&cell| *cell > 0).count() as i32)
            .sum();
        
        return horizontal + vertical + top;
    }
}
