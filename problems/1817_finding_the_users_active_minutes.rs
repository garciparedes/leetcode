use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let k = k as usize; 
        let mut uniques = HashMap::new();
        for log in logs {
            uniques.entry(log[0]).or_insert_with(HashSet::new).insert(log[1]);
        }
        
        let mut ans = vec![0; k];
        for l in uniques.values().map(|s| s.len()).filter(|l| *l > 0) {
            ans[l - 1] += 1;
        }
        return ans;
    }
}
