impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut best = 0;
        let mut last = None;
        for j in 0..32 {
            if (n & (1 << j)) > 0 {
                if let Some(i) = last {
                    if best < j - i {
                        best = j - i;
                    }
                }
                last = Some(j);
            }
        }
        return best;
    }
}
