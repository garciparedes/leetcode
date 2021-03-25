use std::cmp;

impl Solution {
    pub fn maximum_score(mut a: i32, mut b: i32, mut c: i32) -> i32 {
        let mut ans = 0;
        while (a > 0 && b > 0 && c > 0) {
            if (a <= b && b <= c) || (c <= b && b <= a) {
                a -= 1;
                c -= 1;
            } else if (b <= a && a <= c) || (c <= a && a <= b) {
                b -= 1;
                c -= 1;
            } else {
                a -= 1;
                b -= 1;
            }
            ans += 1;
        }
        if a == 0 {
            ans += cmp::min(b, c);
        } else if b == 0 {
            ans += cmp::min(a, c);
        } else {
            ans += cmp::min(a, b);
        }
        return ans;
    }
}
