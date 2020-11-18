use std::cmp;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable();
        let n = intervals.len();
        let mut dropped = vec![false; n];
        let mut i = 0;
        for i in 0..n {
            if dropped[i] {
                continue;
            }
            
            for j in (i + 1)..n {
                if dropped[j] {
                    continue;
                }
                if Self::overlaps(&intervals[i], &intervals[j]) {
                    intervals[i] = vec![
                        cmp::min(intervals[i][0], intervals[j][0]), 
                        cmp::max(intervals[i][1], intervals[j][1]),
                    ];
                    dropped[j] = true;
                }
            }
        }
        return intervals
            .into_iter()
            .enumerate()
            .filter(|(i, _)| !dropped[*i])
            .map(|(_, v)| v).collect();
    }
    
    fn overlaps(a: &[i32], b: &[i32]) -> bool {
        (a[0] <= b[1] && b[0] <= a[1]) 
    }
}
