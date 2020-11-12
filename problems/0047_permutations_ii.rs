impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        
        let mut ans = Vec::new();
        let mut chosen = vec![false; nums.len()];
        let mut current = Vec::new();
        
        Self::rec(&nums, &mut chosen, &mut current, &mut ans);
        
        return ans;
    }
    
    fn rec(nums: &[i32], chosen: &mut [bool], current: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if current.len() == nums.len() {
            ans.push(current.clone());
            return;
        }
        let mut previous = -100;
        for i in 0..nums.len() {
            if chosen[i] {
                continue;
            }
            if previous == nums[i] {
                continue;
            }
            previous = nums[i];
            chosen[i] = true;
            current.push(nums[i]);
            Self::rec(nums, chosen, current, ans);
            current.pop();
            chosen[i] = false;
        }
    }
}
