impl Solution {
    pub fn number_of_matches(mut n: i32) -> i32 {
        let mut ans = 0;
        while n > 1 {
            let tmp = n / 2;
            n -= tmp;
            ans += tmp;
        }
        return ans;
    }
}
