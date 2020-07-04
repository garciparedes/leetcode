impl Solution {
    pub fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
        let mut best = (0, a[0]);
        for (i, item) in a.into_iter().enumerate() {
            if item <= best.1 {
                continue;
            }
            best = (i, item);
        }
        return best.0 as i32;
    }
}
