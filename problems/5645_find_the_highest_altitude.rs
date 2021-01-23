impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut maximum = 0;
        let mut current = 0;
        for change in gain {
            current += change;
            if current > maximum {
                maximum = current;
            }
        }
        return maximum;
    }
}
