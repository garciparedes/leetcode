impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        Self::backtrack(&mut ans, &mut Vec::new(), &nums);
        return ans;
    }
    
    fn backtrack(ans: &mut Vec<Vec<i32>>, current: &mut Vec<i32>, nums: &Vec<i32>) {
        if current.len() == nums.len() {
            ans.push(current.clone());
            return;
        }
        
        for i in 0..nums.len() {
            if current.contains(&nums[i]) {
                continue;
            }
            current.push(nums[i]);
            Self::backtrack(ans, current, nums);
            current.remove(current.len() - 1);
        }
    }
}
