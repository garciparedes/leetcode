impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {        
        let k = k as usize;
        let n = nums.len();
        let k = k % n;
        if k == 0 {
            return;
        }
        
        let mut processed = vec![false; k];
        for i in 0..k {
            Self::rotate_period(nums, k, i, nums[i], &mut processed);
        }
    }
    
    fn rotate_period(nums: &mut Vec<i32>, k: usize, mut index: usize, mut memorized: i32, processed: &mut Vec<bool>) {  
        if processed[index] {
            return;
        }
        processed[index] = true;
        
        let n = nums.len();
        index += k;
        while index < n {
            let tmp = nums[index];
            nums[index] = memorized;
            memorized = tmp;
            index += k;
        }
        index %= n;
        
        let tmp = nums[index];
        nums[index] = memorized;
        
        if processed[index] {
            return;
        }
        Self::rotate_period(nums, k, index, tmp, processed);
    }
    
}
