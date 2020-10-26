use std::mem;
use std::cmp;

impl Solution {
    pub fn are_connected(n: i32, threshold: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut uf = Self::union(n as usize, threshold as usize);
        let ans = queries
            .into_iter()
            .map(|q| Self::find(&mut uf, q[0] as usize) == Self::find(&mut uf, q[1] as usize))
            .collect();
        return ans;
    }
    
    fn union(n: usize, threshold: usize) -> Vec<i32> {
        let mut uf = vec![-1; n + 1];
        for i in (threshold  + 1)..((n / 2) + 1) {
            if uf[i] != - 1 {
                continue;
            }
            let mut p1 = i as i32;
            for j in (2 * i..n + 1).step_by(i) {
                let mut p2 = Self::find(&mut uf, j);
                if p1 == p2 {
                    continue;
                }
                if uf[p1 as usize] > uf[p2 as usize] {
                    mem::swap(&mut p1, &mut p2);
                }
                uf[p1 as usize] += uf[p2 as usize];
                uf[p2 as usize] = p1;
            }
        }
        return uf;
    }
    
    fn find(uf: &mut [i32], i: usize) -> i32 {
        if uf[i] < 0 {
            return i as i32;
        }
        uf[i] = Self::find(uf, uf[i] as usize);
        
        return uf[i];
    }
}
