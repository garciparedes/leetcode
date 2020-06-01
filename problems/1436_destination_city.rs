use std::collections::HashMap;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        
        for path in paths {
            let (origin, destination) = (path[0].clone(), path[1].clone());
            graph.entry(destination.clone()).or_insert_with(Vec::new);
            graph.entry(origin).or_insert_with(Vec::new).push(destination);
        }
        
        for (node, edges) in graph.iter() {
            if !edges.is_empty() {
                continue;
            }
            return node.clone();
        }
        return String::new();
    }
}
