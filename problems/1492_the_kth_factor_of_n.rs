impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        let mut count = 0;
        for value in 1..(n + 1) {
            if n % value == 0 {
                count += 1;
                if count == k {
                    return value;
                }
            }
        }
        return -1;
    }
}
