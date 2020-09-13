use std::cmp;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        
        let mut i = 0;
        while i < intervals.len() && intervals[i][1] < new_interval[0] {
            ans.push(intervals[i].clone());
            i += 1;
        }
        if i == intervals.len() {
            ans.push(new_interval);
            return ans;
        }

        let left = cmp::min(intervals[i][0], new_interval[0]);
        while i < intervals.len() - 1 && intervals[i + 1][0] <= new_interval[1] {
            i += 1;
        }
        let mut right = new_interval[1];
        if intervals[i][0] <= new_interval[1] {
            right = cmp::max(intervals[i][1], new_interval[1]);
            i += 1;
        } 
        ans.push(vec![left, right]);
        
        while i < intervals.len() {
            ans.push(intervals[i].clone());
            i += 1;
        }
        
        return ans;
    }
}
