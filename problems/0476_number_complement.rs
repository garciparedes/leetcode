impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut mask = !0;
        while num & mask != 0 {
            mask <<= 1;
        }
        return !num & !mask;
    }
}
