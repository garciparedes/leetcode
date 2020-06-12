impl Solution {
    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        return a
            .iter()
            .map(|row| 
                row
                    .into_iter()
                    .rev()
                    .map(|&x| if x == 0 { 1 } else { 0 })
                    .collect::<Vec<i32>>()
            )
            .collect();
    }
}
