impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut counter = 0;
        for row in grid.iter().rev() {
            let mut current_counter = 0;
            for &cell in row.iter().rev() {
                if cell > -1 {
                    break;
                }
                current_counter += 1;
            }
            if current_counter == 0 {
                break;
            }
            counter += current_counter;
        }
        return counter;
    }
}
