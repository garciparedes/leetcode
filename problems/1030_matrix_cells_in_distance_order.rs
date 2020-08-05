impl Solution {
    pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut coordinates = Vec::new();
        for i in 0..r {
            for j in 0..c {
                coordinates.push(vec![i, j]);
            }
        }
        coordinates.sort_unstable_by_key(|x| (r0 - x[0]).abs() + (c0 - x[1]).abs());
        
        return coordinates;
    }
}
