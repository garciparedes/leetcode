use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let mut memory = HashMap::new();
        let m = grid[0].len() as i32;
        let result = Self::dfs(&grid, 0, 0, m - 1, &mut memory);
        return result;
    }
    
    fn dfs(
        grid: &Vec<Vec<i32>>, 
        depth: usize, 
        first: i32, 
        second: i32, 
        memory: &mut HashMap<(usize, i32, i32), i32>
    ) -> i32 {
        if first < 0 || second < 0 || first == (grid[0].len() as i32) || second == (grid[0].len() as i32) {
            return -1;
        }
        if depth == grid.len() - 1 {
            return grid[depth][first as usize] + grid[depth][second as usize] * (first != second) as i32;
        }
        if let Some(result) = memory.get(&(depth, first, second)) {
            return *result;
        }
        
        let mut best = 0;
        for i in vec![-1, 0, 1] {
            for j in vec![-1, 0, 1] {
                let current = Self::dfs(grid, depth + 1, first + i, second + j, memory);
                best = cmp::max(best, current);
            }
        }
        let result = (
            grid[depth][first as usize] 
            + grid[depth][second as usize] * (first != second) as i32 
            + best
        );
        memory.insert((depth, first, second), result);
        return result;
    }
}
