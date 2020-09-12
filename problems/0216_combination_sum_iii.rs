impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        Self::backtrack(&mut ans, Vec::new(), 0, n, k as usize); 
        return ans;
    }
    
    fn backtrack(
        ans: &mut Vec<Vec<i32>>, 
        current: Vec<i32>, 
        current_sum: i32, 
        target: i32,
        k: usize,
    ) {
        if current.len() == k {
            if current_sum == target {
                ans.push(current);
            }
            return;
        }
        if current_sum > target {
            return;
        }
        for value in (current.last().unwrap_or(&0) + 1)..10 {
            let mut tmp = current.clone();
            tmp.push(value);
            Self::backtrack(ans, tmp, current_sum + value, target, k);
        }
    }
}
