impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        (0..n)
            .map(|i| 2 * i + start)
            .fold(0, |acc, x| acc ^ x)
    }
}
