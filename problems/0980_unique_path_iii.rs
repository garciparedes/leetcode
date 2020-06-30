impl Solution {
    pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
        let starting = Self::find_starting(&grid);
        let obstacles = Self::count_obstacles(&grid);
        let energy = grid.len() * grid[0].len() - obstacles - 1;
        
        return Self::navigate(&mut grid, starting, energy) as i32;
    }
    
    fn find_starting(grid: &Vec<Vec<i32>>) -> (usize, usize) {
        for (i, row) in grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    return (i, j);
                }
            }
        }
        panic!("Not found!");
    }
    
    fn count_obstacles(grid: &Vec<Vec<i32>>) -> usize {
         return grid
            .iter()
            .map(|row| row.iter().map(|&cell| (cell== -1) as usize).sum::<usize>())
            .sum();
    }
    
    fn navigate(grid: &mut Vec<Vec<i32>>, position: (usize, usize), energy: usize) -> usize {
        if (
            energy < 0
            || position.0 < 0 
            || position.0 >= grid.len()
            || position.1 < 0 
            || position.1 >= grid[0].len()
            || grid[position.0][position.1] == -1 
            || grid[position.0][position.1] == -2
        ) {
            return 0;
        }
        if grid[position.0][position.1] == 2 {
            return (energy == 0) as usize;
        }
        
        grid[position.0][position.1] = -2;
        let result = (
            Self::navigate(grid, (position.0, position.1 + 1), energy - 1) 
            + Self::navigate(grid, (position.0, position.1 - 1), energy - 1) 
            + Self::navigate(grid, (position.0 + 1, position.1), energy - 1) 
            + Self::navigate(grid, (position.0 -1, position.1), energy - 1) 
        );
        grid[position.0][position.1] = 0;
        return result;
    }
}
