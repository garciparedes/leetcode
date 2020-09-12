use std::cmp;

impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let (a, b, c) = (Self::to_binary(a), Self::to_binary(b), Self::to_binary(c));
        let n = cmp::max(a.len(), cmp::max(b.len(), c.len()));
        
        let mut ans = 0;
        for i in (0..n) {
            let a_i =  a.get(i).unwrap_or(&false);
            let b_i =  b.get(i).unwrap_or(&false);
            let c_i =  c.get(i).unwrap_or(&false);
            if (a_i | b_i) != *c_i {
                ans += 1;
                if (a_i & b_i) {
                    ans += 1;
                }
            }
        }
        return ans;
    }
    
    fn to_binary(mut value: i32) -> Vec<bool> {
        let mut ans = Vec::new();
        while value != 0 {
            ans.push(value % 2 == 1);
            value /= 2;
        }
        return ans;
    }
}
