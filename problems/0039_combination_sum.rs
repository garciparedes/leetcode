impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        Self::rec(&candidates, target, 0, 0, Vec::new(), &mut ans);
        return ans;
    }
    
    fn rec(
        candidates: &Vec<i32>, 
        target: i32, 
        index: usize, 
        cum: i32,
        selected: Vec<i32>, 
        ans: &mut Vec<Vec<i32>>
    ) {
        if cum > target {
            return;
        }
        if cum == target {
            ans.push(selected.clone());
            return;
        } 
        if index >= candidates.len() {
            return;
        }
        let mut new_selected = selected.clone();
        new_selected.push(candidates[index]);
        let new_cum = cum + candidates[index];
        Self::rec(candidates, target, index, new_cum, new_selected, ans);
        Self::rec(candidates, target, index + 1, cum, selected.clone(), ans);
    }
}
