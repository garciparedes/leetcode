impl Solution {
    pub fn reinitialize_permutation(n: i32) -> i32 {
        let n = n as usize;
        let target: Vec<_> = (0..n).map(|v| v as i32).collect();
        
        let mut ans = 1;
        let mut perm = target.clone();
        perm = Self::apply_operation(perm);
        while perm != target {
            perm = Self::apply_operation(perm);
            ans += 1;
        }
        return ans;
    }
    
    fn apply_operation(perm: Vec<i32>) -> Vec<i32> {
        let n = perm.len();
        let mut arr = vec![0; n];
        for i in 0..n {
            if i % 2 == 0 {
                arr[i] = perm[i / 2];
            } else {
                arr[i] = perm[n / 2 + (i - 1) / 2];
            }
        }
        return arr;
    }
}
