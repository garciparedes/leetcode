use std::collections::HashSet;

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut mask = 0;
        for i in (0..32).rev() {
            mask |= (1 << i);
            
            let mut set  = HashSet::new();
            for num in nums.iter() {
                set.insert(mask & num);
            }
            
            let tmp = ans | (1 << i);
            for prefix in set.iter() {
                if(set.contains(&(tmp ^ prefix))) {
                    ans = tmp;
                    break;
                }
            }
            
        }
        
        return ans;
    }
}
