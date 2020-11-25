impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        if (k % 2 == 0) || (k % 5 == 0) {
            return -1 
        }
        let mut remainder = 0;
        for n in 1..(k + 1) {
            remainder = (remainder * 10 + 1) % k;
            if remainder == 0 {
                return n;
            }
        }
        return -1;
    }
}
