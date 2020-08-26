impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        return s
            .chars()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, b)| acc + (b as usize - 64) as i32 * i32::pow(26, i as u32));
    }
}
