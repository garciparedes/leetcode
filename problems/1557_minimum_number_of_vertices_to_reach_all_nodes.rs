use std::collections::HashSet;

impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let dependent: HashSet<_> = edges.into_iter().map(|x| x[1]).collect();
        let result: HashSet<_> = (0..n).collect();
        
        return result.difference(&dependent).cloned().collect();
    }
}
