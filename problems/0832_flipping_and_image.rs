impl Solution {
    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        a.into_iter()
            .map(|row| {
                row.into_iter()
                    .rev()
                    .map(|cell| -1 * (cell - 1))
                    .collect()
            })
           .collect()
    }
}
