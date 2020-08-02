use std::cmp;

impl Solution {
    pub fn smallest_range_i(a: Vec<i32>, k: i32) -> i32 {
        let (min, max) = a[1..]
            .into_iter()
            .fold((a[0], a[0]), |acc, &x| (cmp::min(acc.0, x), cmp::max(acc.1, x)));  
        return cmp::max(0, max - min - 2 * k);
    }
}
