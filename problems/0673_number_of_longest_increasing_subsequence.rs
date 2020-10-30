impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut ans, mut max_len) = (0, 0);
        let mut counts = vec![1; n];
        let mut lens = vec![1; n];
        
        for i in 0..n {
            for j in 0..i {
                if !(nums[i] > nums[j]) {
                    continue;
                }
                
                if lens[i] == lens[j] + 1 {
                    counts[i] += counts[j];
                } else if lens[i] < lens[j] + 1 {
                    lens[i] = lens[j] + 1;
                    counts[i] = counts[j];
                }
            }
            
            if max_len == lens[i] {
                ans += counts[i];
            } else if max_len < lens[i] {
                max_len = lens[i];
                ans = counts[i];
            }
        }
        
        return ans;
    }
}
