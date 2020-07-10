impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let num = num as usize;
        let mut result = vec![0; num + 1];
        for i in 1..num + 1 {
            result[i] = result[i & (i - 1)] + 1;
        }
        return result;
    }
}
