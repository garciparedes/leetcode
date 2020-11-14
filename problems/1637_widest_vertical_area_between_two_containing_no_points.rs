use std::collections::HashSet;

impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut xs: Vec<_> = points.into_iter().map(|p| p[0]).collect();
        xs.sort_unstable();
        
        let mut ans = 0;
        for i in 1..xs.len() {
            if (xs[i] - xs[i - 1]) > ans {
                ans = xs[i] - xs[i - 1];
            }
        }
        return ans;
    }
}
