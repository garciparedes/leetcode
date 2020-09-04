use std::collections::HashSet;
use std::cmp;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut s: Vec<_> = s.chars().collect();
        let mut processed = HashSet::new();
        let mut ranges = Vec::new();
        for i in 0..s.len() {
            if processed.contains(&s[i]) {
                continue;
            }
            processed.insert(s[i]);
            let mut j = i;
            for jj in (i..s.len()).rev() {
                if s[i] == s[jj] {
                    j = jj;
                    break;
                }
            }
            ranges.push((i, j));
        }
        
        let mut i = 0;
        while i < ranges.len() - 1 {
            if ranges[i + 1].0 > ranges[i].1 {
                i += 1;
                continue;
            }
            ranges[i] = (ranges[i].0, cmp::max(ranges[i].1, ranges[i + 1].1));
            ranges.remove(i + 1);
        }
        
        return ranges.into_iter().map(|x| (x.1 - x.0 + 1) as i32).collect();
    }
}
