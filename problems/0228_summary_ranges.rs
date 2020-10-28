impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut ans = Vec::new();
        
        let mut i = 0;
        while i < nums.len() {
            let mut j = i;
            while j < nums.len() - 1 && nums[j] == nums[j + 1] - 1 {
                j += 1
            }
            if i == j {
                ans.push(nums[j].to_string());
            } else {
                ans.push(format!("{}->{}", nums[i], nums[j]))
            }
            i = j + 1;
        }
        return ans;
    }
}
