impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let (mut count1, mut count2, mut majority1, mut majority2) = (0, 0, 0, 1);
        for &num in nums.iter() {
            if num == majority1 {
                count1 += 1;
            } else if num == majority2 {
                count2 += 1;
            } else if count1 == 0 {
                majority1 = num;
                count1 += 1;
            } else if count2 == 0 {
                majority2 = num;
                count2 += 1;
            } else {
                count1 -= 1;
                count2 -= 1;
            }
        }
        
        count1 = 0;
        count2 = 0;
        for &num in nums.iter() {
            if num == majority1 {
                count1 += 1;
            } else if num == majority2 {
                count2 += 1;
            }
        }
        
        if count1 > nums.len() / 3 && count2 > nums.len() / 3 {
            return vec![majority1, majority2];
        }
        
        if count1 > nums.len() / 3 {
            return vec![majority1];
        }
        
        if count2 > nums.len() / 3 {
            return vec![majority2];
        }
        
        return Vec::new();
        
    }
}
