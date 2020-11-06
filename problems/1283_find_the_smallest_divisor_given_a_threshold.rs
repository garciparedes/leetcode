impl Solution {
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        let (mut lower, mut upper) = (1, 1_000_000);
        
        while upper - lower > 0 {
            let mid = (lower + upper) / 2;
            let tmp = Self::evaluate(&nums, mid);
            if tmp > threshold {
                lower = mid + 1;
            } else {
                upper = mid;
            }
        }
        
        return upper;
        
        
    }
    
    fn evaluate(nums: &[i32], divisor: i32) -> i32 {
        let mut ans = 0;
        for value in nums {
            ans += value / divisor;
            if value % divisor != 0 {
                ans += 1;
            } 
        }
        return ans;
    }
}
