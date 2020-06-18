impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        for (i, &count) in citations.iter().enumerate() {
            if count >= (n - i) as i32 {
                return (n - i) as i32;
            }
        }
        return 0;
    }
}
