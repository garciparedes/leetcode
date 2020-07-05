impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        format!("{:032b}", x)
            .chars()
            .zip(format!("{:032b}", y).chars())
            .map(|(a, b)| (a != b) as i32)
            .sum()
    }
}
