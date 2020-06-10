impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut result = Vec::new();
        for i in (1..(n / 2 + 1)).rev() {
            result.push(- (i as i32))
        }
        if n % 2 == 1 {
            result.push(0);
        }
        for i in 1..((n / 2) + 1) {
            result.push((i as i32))
        }
        return result;
    }
}
