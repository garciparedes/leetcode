use std::collections::{
    HashMap,
};
use std::cmp::Ordering;
use std::cmp;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let l = tasks.len() as i32;
        let mut counter = HashMap::new();
        for task in tasks {
            *counter.entry(task).or_insert(0) += 1;
        }
        
        let mut counts: Vec<i32> = counter.values().cloned().collect();
        let (maximum, maximum_count) = counts.iter().fold((0, 0), |(tmp_max, tmp_count), &x| {
            match tmp_max.cmp(&x) {
                Ordering::Less => (x, 1),
                Ordering::Equal => (tmp_max, tmp_count + 1),
                Ordering::Greater => (tmp_max, tmp_count),
            }
        });
        
        return cmp::max(l, (maximum - 1) * (n + 1) + maximum_count);
    }
}
