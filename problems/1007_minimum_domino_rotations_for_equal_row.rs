use std::cmp;

impl Solution {
    pub fn min_domino_rotations(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let n = a.len();
        
        let (mut ans_aa, mut ans_ab) = (0, 0);
        for i in 0..n {
            if a[i] != a[0] && b[i] != a[0] {
                break;
            }
            if a[i] != a[0] {
                ans_aa += 1;
            }
            if b[i] != a[0] {
                ans_ab += 1;
            }
            if i == n - 1 {
                return cmp::min(ans_aa, ans_ab);   
            }
        }

        let (mut ans_ba, mut ans_bb) = (0, 0);
        for i in 0..n {
            if a[i] != b[0] && b[i] != b[0] {
                break;
            }
            if a[i] != b[0] {
                ans_ba += 1;
            }
            if b[i] != b[0] {
                ans_bb += 1;
            }
            if i == n - 1 {
                return cmp::min(ans_ba, ans_bb);   
            }
        }
        
        return -1;

    }
}
