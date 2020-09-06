use std::collections::HashSet;
use std::cmp;

impl Solution {
    pub fn largest_overlap(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> i32 {
        let n = a.len() as isize;
        let a_ones = Self::build_ones(&a);
        let b_ones = Self::build_ones(&b);
        
        let mut best = 0;
        for i in (-n + 1)..n {
            for j in (-n + 1)..n {
                let current = b_ones
                    .iter()
                    .filter(|&(ii, jj)| a_ones.contains(&(ii + i, jj + j)))
                    .count();
                best = cmp::max(best, current);
            }
        }
        return best as i32;
    }
    
    fn build_ones(data: &Vec<Vec<i32>>) -> HashSet<(isize, isize)> {
        let n = data.len();
        let mut ones = HashSet::new(); 
        for i in 0..n {
            for j in 0..n {
                if data[i][j] == 1 {
                    ones.insert((i as isize, j as isize));
                }
            }
        }
        return ones;
    } 
}
