use std::cmp;
use std::collections::{
    HashSet,
    HashMap,   
};

impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut adj = HashMap::new();
        
        for edge in roads {
            adj.entry(edge[0]).or_insert_with(HashSet::new).insert(edge[1]);
            adj.entry(edge[1]).or_insert_with(HashSet::new).insert(edge[0]);
        }
        
        let mut ans = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                let mut count = 0;
                count += adj.entry(i).or_insert_with(HashSet::new).len();
                count += adj.entry(j).or_insert_with(HashSet::new).len();
                if adj.entry(i).or_insert_with(HashSet::new).contains(&j) {
                    count -= 1;
                }
                ans = cmp::max(ans, count);
            }
        }
        return ans as i32;
    }
}
