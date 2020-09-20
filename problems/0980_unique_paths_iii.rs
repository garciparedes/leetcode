impl Solution {
    pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {        
        let mut starting = (-1, -1);
        let mut ending = (-1, -1);
        let mut obstacles = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                match grid[i][j] {
                    1 => starting = (i as isize, j as isize),
                    2 => ending = (i as isize, j as isize),
                    -1 => obstacles += 1,
                    _ => (),
                } 
            }
        }
        return Self::rec(&mut grid, starting, ending, obstacles + 1);
    }
    
    fn rec(
        grid: &mut Vec<Vec<i32>>, 
        position: (isize, isize), 
        ending: (isize, isize), 
        steps: usize,
    ) -> i32 {
            if (position.0 < 0 
                || position.1 < 0 
                || position.0 >= grid.len() as isize 
                || position.1 >= grid[0].len() as isize 
                || grid[position.0 as usize][position.1 as usize] == 3
                || grid[position.0 as usize][position.1 as usize] == -1
            ) {
                return 0;
            }
            
            if position == ending && (steps == grid.len() * grid[0].len()) {
                return 1;
            }
            
            grid[position.0 as usize][position.1 as usize] = 3;
            let ans = (
                Self::rec(grid, (position.0, position.1 - 1), ending, steps + 1) 
                + Self::rec(grid, (position.0, position.1 + 1), ending, steps + 1) 
                + Self::rec(grid, (position.0 - 1, position.1), ending, steps + 1) 
                + Self::rec(grid, (position.0 + 1, position.1), ending, steps + 1)
            );
            grid[position.0 as usize][position.1 as usize] = 0;
            return ans;
        }
}
