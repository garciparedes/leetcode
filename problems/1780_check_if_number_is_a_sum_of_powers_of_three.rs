use std::cmp::Ordering;

impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        let powers = Self::generate_powers(n);
        return Self::helper(n, &powers, 0, 0);
    }
    
    fn generate_powers(n: i32) -> Vec<i32> {
        let mut ans = vec![1];
        while ans[ans.len() - 1] < n {
            let tmp = ans[ans.len() - 1] * 3;
            ans.push(tmp);
        }
        return ans;
    }
    
    fn helper(target: i32, powers: &[i32], i: usize, cum: i32) -> bool {
        match cum.cmp(&target) {
            Ordering::Less => {
                if i == powers.len() {
                    return false;
                }
                return (
                    Self::helper(target, powers, i + 1, cum) 
                    || Self::helper(target, powers, i + 1, cum + powers[i])
                );
            },
            Ordering::Equal => true,
            Ordering::Greater => false,
        }
    }
}
