impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let mut perm = vec![false; n as usize];
        
        return Self::helper(&mut perm, 0);
    }
    
    fn helper(perm: &mut [bool], l: usize) -> i32 {
        if perm.len() == l {
            return 1;
        }
        
        let mut ans = 0;
        for i in 0..perm.len() {
            if perm[i] {
                continue;
            }
            if (l + 1) % (i + 1) != 0 && (i + 1) % (l + 1) != 0 {
                continue;
            }
            perm[i] = true;
            ans += Self::helper(perm, l + 1);
            perm[i] = false;
        }
        return ans;
    }
}
