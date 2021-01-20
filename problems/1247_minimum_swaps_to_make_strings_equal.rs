use std::mem;

impl Solution {
    pub fn minimum_swap(s1: String, s2: String) -> i32 {
        let (mut x_y, mut y_x) = (0, 0);
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 == c2 {
                continue;
            }
            if c1 == 'x' {
                x_y += 1;
            } else {
                y_x += 1;
            }
        }
        
        if (x_y + y_x) % 2 == 1 {
            return -1;
        }
        
        let mut ans = x_y / 2 + y_x / 2;
        if x_y % 2 == 1 {
            ans += 2;
        }
        return ans;
    }
}
