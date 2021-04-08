impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let coordinates: Vec<_> = coordinates.chars().collect();
        let x = coordinates[0] as u8 - 'a' as u8;
        let y = coordinates[1].to_digit(10).unwrap();
        return (x % 2 == 0) == (y % 2 == 0);
    }
}
