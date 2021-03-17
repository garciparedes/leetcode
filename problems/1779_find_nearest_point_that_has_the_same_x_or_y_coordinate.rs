impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut best = None;
        
        for (i, p) in points.into_iter().enumerate() {
            if p[0] == x || p[1] == y {
                let tmp = (p[0] - x).abs() + (p[1] - y).abs();
                if let Some((_, b)) = best {
                    if tmp < b {
                        best = Some((i, tmp));
                    }
                } else {
                    best = Some((i, tmp));
                }
            }
        }
        
        if let Some((ans, _)) = best {
            return ans as i32;
        } else {
            return -1;
        }
    }
}
