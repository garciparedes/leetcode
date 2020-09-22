impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut majority = nums[0];
        for num in nums {
            if count == 0 {
                majority = num;
                count += 1;
            } else {
                if num == majority {
                    count += 1;
                } else {
                    count -= 1;
                }    
            } 
        }
        return majority;
    }
}
