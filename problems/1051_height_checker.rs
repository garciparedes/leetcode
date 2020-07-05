impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut expected = heights.clone();
        expected.sort();
        
        return heights
            .into_iter()
            .zip(expected)
            .map(|(a, b)| (a != b) as i32)
            .sum();
    }
}
