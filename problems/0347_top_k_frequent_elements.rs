use std::collections::{
    HashMap,
    BinaryHeap,
};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counter = HashMap::new();
        for num in nums {
            *counter.entry(num).or_insert(0) += 1;
        }
        
        let mut heap: BinaryHeap<_> = counter
            .into_iter()
            .map(|(num, count)| (count, num))
            .collect();
        
        return (0..k)
            .map(|_| heap.pop().unwrap().1)
            .collect();
    }
}
