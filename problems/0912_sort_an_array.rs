impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        Self::helper(&nums)
    }
    
    fn helper(nums: &[i32]) -> Vec<i32> {
        if nums.len() < 2 {
            return nums.to_vec();
        } else {
            let mid = nums.len() / 2;
            
            let base = Self::helper(&nums[..mid]);
            let another = Self::helper(&nums[mid..]);
            
            return Self::merge(base, another);
        }
    }
    
    fn merge(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut a_iter = a.into_iter().peekable();
        let mut b_iter = b.into_iter().peekable();
    
        let mut ans = Vec::new();
        loop {
            let v = match (a_iter.peek(), b_iter.peek()) {
                (Some(a_val), Some(b_val)) => {
                    if a_val < b_val { 
                        a_iter.next().unwrap() 
                    } else { 
                        b_iter.next().unwrap() 
                    }
                },
                (Some(_), None) => a_iter.next().unwrap(),
                (None, Some(_)) => b_iter.next().unwrap(),
                (None, None) => break,
            };
            ans.push(v);
        }
        return ans;
    }
}
