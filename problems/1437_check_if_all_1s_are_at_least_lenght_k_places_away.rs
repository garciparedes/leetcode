impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut last: Option<usize> = None;
        for (i, num) in nums.into_iter().enumerate() {
            if num == 0 {
                continue;
            }
            if let Some(j) = last.as_mut() {
                if (i - (*j + 1)) < k {
                    return false;
                }
                *j = i;
            } else {
                last = Some(i);
            }
        }
        return true;
    }
}
