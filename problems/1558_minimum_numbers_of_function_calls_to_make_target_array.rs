impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable_by_key(|x| -x);
        
        let mut count = 0;
        while !nums.is_empty() {
            let mut tmp = false;
            
            let mut tmp = false;
            for i in 0..nums.len() {
                if nums[i] % 2 == 1 {                     
                    nums[i] -= 1;
                    count += 1;
                    tmp = true;
                } 
            }
            if !tmp {
                count += 1;
                for i in 0..nums.len() {                     
                    nums[i] /= 2;
                }
            }
            while !nums.is_empty() && nums[nums.len() - 1] == 0 {
                nums.pop();
            }
        }
        return count;
        
    }
}
