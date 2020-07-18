use std::collections::{
    HashMap,
    HashSet,
    VecDeque,
};

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = HashMap::new();
        for edge in prerequisites.into_iter() {
            graph.entry(edge[1]).or_insert_with(HashSet::new).insert(edge[0]);
        }
        
        let mut queue: VecDeque<i32> = (0..num_courses)
            .filter(|node| !graph.values().any(|edges| edges.contains(node)))
            .collect();
        
        let mut result = Vec::new();
        while let Some(node) = queue.pop_front() {
            if let Some(current_edges) = graph.remove(&node) {
                for item in current_edges {
                    if graph.values().any(|edges| edges.contains(&item)) {
                        continue;
                    }
                    queue.push_back(item);
                }
                
            }
            result.push(node)
        } 
        if !graph.is_empty() {
            return Vec::new()
        }
        
        return result;
    }
}
