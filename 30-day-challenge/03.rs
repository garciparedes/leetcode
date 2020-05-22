use std::cmp;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut maximum: i32 = 0;
        let mut positive: i32 = 0;
        let mut negative: i32 = 0;
        let mut j: usize = 0;
        
        for (i, &v) in nums.iter().enumerate() {
            if v < 0 {
                negative -= v;
            } else {
                positive += v;   
            }
            while j <= i && positive < negative {
                let vv = nums[j];
                j += 1;
                if vv < 0 {
                    negative += vv;
                } else {
                    positive -= vv;   
                }
            }
            maximum = cmp::max(maximum, positive - negative);
        }
        return maximum;
    }
}
