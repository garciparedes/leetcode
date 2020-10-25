impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        l.into_iter()
            .zip(r.into_iter())
            .map(|(i, j)| {
                if j - i < 2 {
                    return true;
                }
                let (i, j) = (i as usize, j as usize);
                let mut tmp = (&nums[i..j + 1]).to_vec();
                tmp.sort_unstable();
                let mut d = tmp[1] - tmp[0];
                for k in 0..tmp.len() - 1 {
                    if tmp[k + 1] - tmp[k] != d {
                        return false;
                    }
                } 
                return true;
            })
            .collect()
    }
}
