impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let mut mask = !0;
        while n & mask != 0 {
            mask <<= 1;
        }
        return !n & !mask;
    }
}
