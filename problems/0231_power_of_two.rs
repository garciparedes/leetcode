impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n < 1 {
            return false;
        }
        let mut one = false;
        for c in format!("{:b}", n).chars() {
            if c == '1' {
                if one {
                    return false;
                }
                one = true;
            }
        }
        return one;
    }
}
