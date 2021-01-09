impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut ans = 0;
        let mut prev = 1;
        let mut last_monday = 0;
        for i in 0..n {
            if i % 7 == 0 {
                last_monday += 1;
                prev = last_monday;
            } else {
                prev += 1;
            }
            ans += prev;
        }
        return ans;
    }
}
