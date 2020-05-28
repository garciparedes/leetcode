use std::cmp;

impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let first_axis = Solution::first_axis_max(&grid);
        let second_axis = Solution::second_axis_max(&grid);
        
        let mut counter = 0;
        
        for (i, row) in grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                counter += cmp::min(first_axis[i], second_axis[j]) - cell
            }
        }
        
        return counter;
    }
        
    fn first_axis_max(grid: &Vec<Vec<i32>>) -> Vec<i32> {
        return grid.iter().map(|x| x.iter().max().unwrap()).cloned().collect();
    }
    
    fn second_axis_max(grid: &Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        for i in 0..grid[0].len() {
            let mut current = 0;
            for row in grid {
                current = cmp::max(current, row[i]);
            }
            result.push(current);
        }
        return result;
    }
}
