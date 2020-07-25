impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut previous = 0;
        for v in target {
            if (v - previous) > 0 {
                count += (v - previous);
            }
            previous = v;
        }
        return count;
    }
}
