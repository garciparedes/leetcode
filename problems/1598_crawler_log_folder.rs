use std::cmp;

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut count = 0;
        for log in logs {
            match log.as_str() {
                "../" => count = cmp::max(0, count - 1),
                "./" => (),
                _ => count += 1,
            }
        }
        return count;
    }
}
