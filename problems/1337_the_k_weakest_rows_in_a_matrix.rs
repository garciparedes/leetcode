use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let m = mat[0].len() as usize;
        let mut heap: BinaryHeap<_> = mat
            .into_iter()
            .enumerate()
            .map(|(i, row)| (
                Reverse(row.into_iter().position(|cell| cell == 0).unwrap_or(m)),  
                Reverse(i as i32)
            ))
            .collect();        
        
        return (0..k)
            .map(|_| match heap.pop() {
                Some((_, Reverse(i))) => i,
                None => panic!(),
            })
            .collect();
    }
}
