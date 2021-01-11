impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut ans = vec![first];
        for e in encoded {
            ans.push(ans[ans.len() - 1] ^ e);
        }
        return ans;
    }
}
