use std::cmp;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let nums: Vec<_> = nums.into_iter().map(|x| x as i64).collect();
        let t = t as i64;
        let k = cmp::min(k as usize + 1, nums.len());
        
        let mut current = nums[..k].to_vec();
        current.sort_unstable();
        
        if Self::validate(&current, t) {
            return true;
        }

        if k >= nums.len() {
            return false;
        }
        
        for i in k..nums.len() {
            let mut j = 0;
            while current[j] != nums[i - k] {
                j += 1;
            }
            current.remove(j);
            
            let mut j = 0;
            while j < current.len() && current[j] < nums[i] {
                j += 1;
            }
            current.insert(j, nums[i]);
            
            if Self::validate(&current, t) {
                return true;
            }
        }

        return false;
    }
    
    fn validate(current: &Vec<i64>, t: i64) -> bool {
        if current.is_empty() {
            return false;
        }
        for i in 0..current.len() - 1 {
            if current[i + 1] - current[i] <= t {
                return true;   
            }
        }
        return false;
    }
}
