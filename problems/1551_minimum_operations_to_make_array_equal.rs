impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        let mut result = 0;
        for v in (1..n).step_by(2) {
            result += n - v;
        }
        return result;
    }
}
