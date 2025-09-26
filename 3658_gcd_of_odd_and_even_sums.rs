impl Solution {
    pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
        let s_0 = Self::incremental_sum(n);
        let s_1 = Self::incremental_sum(n - 1);
        Self::gcd(s_0 + s_1, 2 * s_0)
    }

    fn incremental_sum(n: i32) -> i32 {
        n * (n + 1) / 2
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while a != b {
            if a > b {
                a -= b
            } else {
                b -= a
            }
        }
        a
    }
}
